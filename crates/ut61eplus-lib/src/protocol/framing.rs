//! Shared frame extraction and read-loop functions for protocols using ABCD headers.

use crate::error::{Error, Result};
use crate::transport::Transport;
use log::{debug, trace, warn};

/// How to handle frame extraction errors (checksum mismatches).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FrameErrorRecovery {
    /// Propagate the error immediately (UT61E+ behavior).
    Propagate,
    /// Skip past the current ABCD header and retry (UT8803/UT171/UT181A behavior).
    SkipAndRetry,
}

/// Shared read loop: extract frames from `rx_buf`, reading more data from
/// `transport` when needed.
///
/// - `extract_fn`: protocol-specific frame extractor (e.g. `extract_frame_abcd_be16`).
/// - `accept_fn`: predicate on the extracted payload. Return `true` to accept
///   the frame, `false` to skip it and continue reading (used by UT171/UT181A
///   to filter non-measurement frames).
/// - `recovery`: whether to skip-and-retry on frame errors or propagate them.
/// - `label`: protocol label for log messages (e.g. `"ut8803"`).
///
/// Constants match the values used by all four protocol implementations:
/// `READ_TIMEOUT_MS = 2000`, `MAX_ATTEMPTS = 64`.
pub(crate) fn read_frame<F, A>(
    rx_buf: &mut Vec<u8>,
    transport: &dyn Transport,
    extract_fn: F,
    accept_fn: A,
    recovery: FrameErrorRecovery,
    label: &str,
) -> Result<Vec<u8>>
where
    F: Fn(&[u8]) -> Result<Option<(Vec<u8>, usize)>>,
    A: Fn(&[u8]) -> bool,
{
    const READ_TIMEOUT_MS: i32 = 2000;
    const MAX_ATTEMPTS: usize = 64;

    for _ in 0..MAX_ATTEMPTS {
        match extract_fn(rx_buf) {
            Ok(Some((payload, consumed))) => {
                rx_buf.drain(..consumed);
                if accept_fn(&payload) {
                    return Ok(payload);
                }
                debug!(
                    "{label}: skipping non-matching frame ({} bytes): {:02X?}",
                    payload.len(),
                    &payload[..payload.len().min(4)]
                );
            }
            Ok(None) => {
                let mut tmp = [0u8; 64];
                let n = transport.read_timeout(&mut tmp, READ_TIMEOUT_MS)?;
                if n == 0 {
                    return Err(Error::Timeout);
                }
                rx_buf.extend_from_slice(&tmp[..n]);
            }
            Err(e) => match recovery {
                FrameErrorRecovery::Propagate => return Err(e),
                FrameErrorRecovery::SkipAndRetry => {
                    warn!("{label}: frame error: {e}, skipping");
                    if let Some(pos) = rx_buf.windows(2).position(|w| w == HEADER) {
                        rx_buf.drain(..pos + 2);
                    } else {
                        rx_buf.clear();
                    }
                }
            },
        }
    }

    Err(Error::Timeout)
}

/// Header bytes shared across all UNI-T protocols.
pub const HEADER: [u8; 2] = [0xAB, 0xCD];

/// Minimum valid response length: header(2) + length(1) + checksum(2) = 5
/// (length byte value must be >= 2 to hold at least the checksum)
const MIN_RESPONSE_LEN: usize = 5;

/// Extract a frame using UT61E+ format: AB CD len payload checksum_BE.
///
/// Length byte counts everything after itself (payload + 2-byte checksum).
/// Checksum is 16-bit BE sum of all bytes before the checksum.
///
/// Returns `Ok(Some((payload, consumed)))` if a valid frame is found,
/// `Ok(None)` if incomplete, `Err` on checksum mismatch.
pub fn extract_frame_abcd_be16(buf: &[u8]) -> Result<Option<(Vec<u8>, usize)>> {
    let Some(start) = buf.windows(2).position(|w| w == HEADER) else {
        return Ok(None);
    };

    let remaining = &buf[start..];
    if remaining.len() < MIN_RESPONSE_LEN {
        return Ok(None);
    }

    // Byte after header is the "length" — counts everything after itself,
    // i.e. payload + 2-byte checksum. Verified against real device traces.
    let len_byte = remaining[2] as usize;
    if len_byte < 2 {
        return Ok(None);
    }
    let frame_len = 2 + 1 + len_byte; // header + len_byte + (payload + checksum)
    let payload_len = len_byte - 2;

    if remaining.len() < frame_len {
        return Ok(None);
    }

    let frame = &remaining[..frame_len];
    trace!("framing: raw frame: {:02X?}", frame);

    // Checksum: 16-bit BE sum of all bytes except the last two
    let data_bytes = &frame[..frame_len - 2];
    let computed: u16 = data_bytes.iter().map(|&b| b as u16).sum();
    let received = u16::from_be_bytes([frame[frame_len - 2], frame[frame_len - 1]]);

    if computed != received {
        debug!(
            "framing: checksum mismatch: computed={computed:#06x}, received={received:#06x}, frame={frame:02X?}"
        );
        return Err(Error::ChecksumMismatch {
            expected: received,
            actual: computed,
        });
    }

    let payload = frame[3..3 + payload_len].to_vec();
    let consumed = start + frame_len;

    debug!("framing: valid frame, payload_len={payload_len}, consumed={consumed}");
    Ok(Some((payload, consumed)))
}

/// Expected payload length for a UT61E+ measurement response.
pub const UT61EPLUS_MEASUREMENT_PAYLOAD_LEN: usize = 14;

/// Extract a frame using UT8803 format: AB CD byte2 0x02 payload chk_hi chk_lo.
///
/// Fixed 21-byte frame. Checksum is alternating-byte sum, stored BE at bytes 19-20.
///
/// Returns `Ok(Some((payload, consumed)))` where payload is bytes 2..19 (17 bytes),
/// `Ok(None)` if incomplete.
pub fn extract_frame_ut8803(buf: &[u8]) -> Result<Option<(Vec<u8>, usize)>> {
    const FRAME_LEN: usize = 21;

    let Some(start) = buf.windows(2).position(|w| w == HEADER) else {
        return Ok(None);
    };

    let remaining = &buf[start..];
    if remaining.len() < FRAME_LEN {
        return Ok(None);
    }

    // Byte 3 must be 0x02 (measurement response type)
    if remaining[3] != 0x02 {
        // Not a measurement frame; skip past this header
        debug!("framing: ut8803 byte3={:#04x}, expected 0x02", remaining[3]);
        return Ok(None);
    }

    let frame = &remaining[..FRAME_LEN];
    trace!("framing: ut8803 raw frame: {:02X?}", frame);

    // Checksum: sum of bytes 0..19, stored BE at bytes 19-20.
    // The RE spec describes this as an "alternating-byte sum" (even/odd
    // accumulators), but that's equivalent to a straight sequential sum.
    let mut sum: u16 = 0;
    for &b in &frame[..19] {
        sum = sum.wrapping_add(b as u16);
    }
    let received = u16::from_be_bytes([frame[19], frame[20]]);

    if sum != received {
        debug!(
            "framing: ut8803 checksum mismatch: computed={sum:#06x}, received={received:#06x}, frame={frame:02X?}"
        );
        return Err(Error::ChecksumMismatch {
            expected: received,
            actual: sum,
        });
    }

    // Payload = bytes 2..19 (everything between header and checksum)
    let payload = frame[2..19].to_vec();
    let consumed = start + FRAME_LEN;

    debug!("framing: ut8803 valid frame, consumed={consumed}");
    Ok(Some((payload, consumed)))
}

/// Extract a frame using UT171 format: AB CD len payload chk_lo chk_hi.
///
/// Length is a 1-byte uint8 counting payload bytes only (NOT including checksum).
/// Checksum is 16-bit LE sum of bytes from offset 2 through end of payload
/// (covers length byte + payload, excludes header and checksum).
///
/// Total frame: header(2) + length(1) + payload(N) + checksum(2) = N + 5.
pub fn extract_frame_abcd_1byte_le16(buf: &[u8]) -> Result<Option<(Vec<u8>, usize)>> {
    let Some(start) = buf.windows(2).position(|w| w == HEADER) else {
        return Ok(None);
    };

    let remaining = &buf[start..];
    if remaining.len() < 5 {
        // header(2) + length(1) + checksum(2) minimum
        return Ok(None);
    }

    let payload_len = remaining[2] as usize;
    let frame_len = 2 + 1 + payload_len + 2; // header + length_byte + payload + checksum

    if remaining.len() < frame_len {
        return Ok(None);
    }

    let frame = &remaining[..frame_len];
    trace!("framing: 1byte_le16 raw frame: {:02X?}", frame);

    // Checksum: 16-bit LE sum of bytes[2..frame_len-2] (length byte + payload)
    let checksum_range = &frame[2..frame_len - 2];
    let computed: u16 = checksum_range.iter().map(|&b| b as u16).sum();
    let received = u16::from_le_bytes([frame[frame_len - 2], frame[frame_len - 1]]);

    if computed != received {
        debug!(
            "framing: 1byte_le16 checksum mismatch: computed={computed:#06x}, received={received:#06x}, frame={frame:02X?}"
        );
        return Err(Error::ChecksumMismatch {
            expected: received,
            actual: computed,
        });
    }

    let payload = frame[3..3 + payload_len].to_vec();
    let consumed = start + frame_len;

    debug!("framing: 1byte_le16 valid frame, payload_len={payload_len}, consumed={consumed}");
    Ok(Some((payload, consumed)))
}

/// Extract a frame using UT181A format: AB CD len_lo len_hi payload chk_lo chk_hi.
///
/// Length is 2-byte LE uint16 = payload_size + 2 (includes checksum bytes).
/// Checksum is 16-bit LE sum of bytes from offset 2 through end of payload
/// (covers length field + payload, excludes header and checksum).
pub fn extract_frame_abcd_2byte_le16(buf: &[u8]) -> Result<Option<(Vec<u8>, usize)>> {
    let Some(start) = buf.windows(2).position(|w| w == HEADER) else {
        return Ok(None);
    };

    let remaining = &buf[start..];
    if remaining.len() < 6 {
        // header(2) + length(2) + checksum(2) minimum
        return Ok(None);
    }

    let len_val = u16::from_le_bytes([remaining[2], remaining[3]]) as usize;
    if len_val < 2 {
        return Ok(None);
    }

    let payload_len = len_val - 2;
    let frame_len = 2 + 2 + payload_len + 2; // header + length_field + payload + checksum

    if remaining.len() < frame_len {
        return Ok(None);
    }

    let frame = &remaining[..frame_len];
    trace!("framing: 2byte_le16 raw frame: {:02X?}", frame);

    // Checksum: 16-bit LE sum of bytes[2..frame_len-2] (length field + payload)
    let checksum_range = &frame[2..frame_len - 2];
    let computed: u16 = checksum_range.iter().map(|&b| b as u16).sum();
    let received = u16::from_le_bytes([frame[frame_len - 2], frame[frame_len - 1]]);

    if computed != received {
        debug!(
            "framing: 2byte_le16 checksum mismatch: computed={computed:#06x}, received={received:#06x}, frame={frame:02X?}"
        );
        return Err(Error::ChecksumMismatch {
            expected: received,
            actual: computed,
        });
    }

    let payload = frame[4..4 + payload_len].to_vec();
    let consumed = start + frame_len;

    debug!("framing: 2byte_le16 valid frame, payload_len={payload_len}, consumed={consumed}");
    Ok(Some((payload, consumed)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::mock::MockTransport;

    /// Build a valid UT61E+ frame from a payload.
    fn make_frame_be16(payload: &[u8]) -> Vec<u8> {
        let len_byte = (payload.len() + 2) as u8;
        let mut frame = vec![0xAB, 0xCD, len_byte];
        frame.extend_from_slice(payload);
        let sum: u16 = frame.iter().map(|&b| b as u16).sum();
        frame.push((sum >> 8) as u8);
        frame.push((sum & 0xFF) as u8);
        frame
    }

    #[test]
    fn extract_valid_frame() {
        let payload = vec![0x01, 0x02, 0x03];
        let frame = make_frame_be16(&payload);
        let result = extract_frame_abcd_be16(&frame).unwrap().unwrap();
        assert_eq!(result.0, payload);
        assert_eq!(result.1, frame.len());
    }

    #[test]
    fn extract_with_leading_garbage() {
        let payload = vec![0x01, 0x02, 0x03];
        let frame = make_frame_be16(&payload);
        let mut buf = vec![0xFF, 0xFE, 0xFD];
        buf.extend_from_slice(&frame);
        let result = extract_frame_abcd_be16(&buf).unwrap().unwrap();
        assert_eq!(result.0, payload);
        assert_eq!(result.1, 3 + frame.len());
    }

    #[test]
    fn extract_incomplete() {
        let frame = vec![0xAB, 0xCD, 0x03, 0x01]; // incomplete
        assert!(extract_frame_abcd_be16(&frame).unwrap().is_none());
    }

    #[test]
    fn extract_bad_checksum() {
        let mut frame = make_frame_be16(&[0x01, 0x02, 0x03]);
        let last = frame.len() - 1;
        frame[last] ^= 0xFF;
        assert!(extract_frame_abcd_be16(&frame).is_err());
    }

    #[test]
    fn extract_no_header() {
        let buf = vec![0x00, 0x01, 0x02, 0x03];
        assert!(extract_frame_abcd_be16(&buf).unwrap().is_none());
    }

    #[test]
    fn extract_real_device_frame() {
        // Real frame captured from UT61E+ on DC mV mode, reading " 0.0004"
        let frame = vec![
            0xAB, 0xCD, 0x10, 0x02, 0x30, 0x20, 0x30, 0x2E, 0x30, 0x30, 0x30, 0x34, 0x00, 0x02,
            0x30, 0x30, 0x30, 0x03, 0x8E,
        ];
        let (payload, consumed) = extract_frame_abcd_be16(&frame).unwrap().unwrap();
        assert_eq!(consumed, 19);
        assert_eq!(payload.len(), 14);
        assert_eq!(payload[0], 0x02);
        assert_eq!(payload[1] & 0x0F, 0x00);
        assert_eq!(&payload[2..9], b" 0.0004");
    }

    #[test]
    fn ut8803_valid_frame() {
        // Construct a minimal valid 21-byte UT8803 frame
        let mut frame = vec![
            0xAB, 0xCD, // header
            0x00, // byte 2
            0x02, // type = measurement
            0x01, // mode
            0x31, // range (with 0x30 prefix)
            0x00, // padding
            b'1', b'2', b'.', b'3', b'4', // display (5 bytes)
            0x00, 0x00, // flags0
            0x00, 0x00, // flags1
            0x00, 0x00, // flags2
            0x00, // flags3
        ];
        // Compute checksum: sum of bytes 0..19
        let sum: u16 = frame.iter().map(|&b| b as u16).sum();
        frame.push((sum >> 8) as u8);
        frame.push((sum & 0xFF) as u8);
        assert_eq!(frame.len(), 21);

        let (payload, consumed) = extract_frame_ut8803(&frame).unwrap().unwrap();
        assert_eq!(consumed, 21);
        assert_eq!(payload.len(), 17); // bytes 2..19
    }

    #[test]
    fn ut8803_incomplete() {
        let buf = vec![0xAB, 0xCD, 0x00, 0x02, 0x01]; // too short
        assert!(extract_frame_ut8803(&buf).unwrap().is_none());
    }

    #[test]
    fn le16_frame_ut181a() {
        // Build a valid UT181A frame (2-byte LE length = payload + 2)
        let payload = vec![0x02, 0x00, 0x11, 0x31]; // type + some data
        let len_val = (payload.len() + 2) as u16; // payload + checksum
        let mut frame = vec![0xAB, 0xCD];
        frame.push((len_val & 0xFF) as u8);
        frame.push((len_val >> 8) as u8);
        frame.extend_from_slice(&payload);
        // Checksum over bytes[2..frame.len()] = length + payload
        let sum: u16 = frame[2..].iter().map(|&b| b as u16).sum();
        frame.push((sum & 0xFF) as u8);
        frame.push((sum >> 8) as u8);

        let (p, consumed) = extract_frame_abcd_2byte_le16(&frame).unwrap().unwrap();
        assert_eq!(p, payload);
        assert_eq!(consumed, frame.len());
    }

    #[test]
    fn le16_frame_ut171() {
        // Build a valid UT171 frame (1-byte length = payload size, LE checksum)
        let payload = vec![0x00, 0x02, 0x80, 0x01, 0x0A, 0x01];
        let len_byte = payload.len() as u8;
        let mut frame = vec![0xAB, 0xCD, len_byte];
        frame.extend_from_slice(&payload);
        // Checksum: LE sum of bytes[2..] (length byte + payload)
        let sum: u16 = frame[2..].iter().map(|&b| b as u16).sum();
        frame.push((sum & 0xFF) as u8);
        frame.push((sum >> 8) as u8);

        let (p, consumed) = extract_frame_abcd_1byte_le16(&frame).unwrap().unwrap();
        assert_eq!(p, payload);
        assert_eq!(consumed, frame.len());
    }

    // --- read_frame tests ---

    #[test]
    fn read_frame_single_chunk() {
        let payload = vec![0x01, 0x02, 0x03];
        let frame = make_frame_be16(&payload);
        let mock = MockTransport::new(vec![frame]);
        let mut rx_buf = Vec::new();

        let result = read_frame(
            &mut rx_buf,
            &mock,
            extract_frame_abcd_be16,
            |_| true,
            FrameErrorRecovery::Propagate,
            "test",
        )
        .unwrap();
        assert_eq!(result, payload);
        assert!(rx_buf.is_empty());
    }

    #[test]
    fn read_frame_split_across_reads() {
        let payload = vec![0x01, 0x02, 0x03];
        let frame = make_frame_be16(&payload);
        // Split the frame into two parts
        let part1 = frame[..3].to_vec();
        let part2 = frame[3..].to_vec();
        let mock = MockTransport::new(vec![part1, part2]);
        let mut rx_buf = Vec::new();

        let result = read_frame(
            &mut rx_buf,
            &mock,
            extract_frame_abcd_be16,
            |_| true,
            FrameErrorRecovery::Propagate,
            "test",
        )
        .unwrap();
        assert_eq!(result, payload);
    }

    #[test]
    fn read_frame_timeout_when_no_data() {
        let mock = MockTransport::new(vec![]);
        let mut rx_buf = Vec::new();

        let result = read_frame(
            &mut rx_buf,
            &mock,
            extract_frame_abcd_be16,
            |_| true,
            FrameErrorRecovery::Propagate,
            "test",
        );
        assert!(matches!(result, Err(Error::Timeout)));
    }

    #[test]
    fn read_frame_propagate_error() {
        // Build a frame with a corrupted checksum
        let mut frame = make_frame_be16(&[0x01, 0x02, 0x03]);
        let last = frame.len() - 1;
        frame[last] ^= 0xFF;
        let mock = MockTransport::new(vec![frame]);
        let mut rx_buf = Vec::new();

        let result = read_frame(
            &mut rx_buf,
            &mock,
            extract_frame_abcd_be16,
            |_| true,
            FrameErrorRecovery::Propagate,
            "test",
        );
        assert!(matches!(result, Err(Error::ChecksumMismatch { .. })));
    }

    #[test]
    fn read_frame_skip_and_retry_on_error() {
        // First frame has bad checksum, second is valid
        let mut bad_frame = make_frame_be16(&[0x01, 0x02, 0x03]);
        let last = bad_frame.len() - 1;
        bad_frame[last] ^= 0xFF;

        let good_payload = vec![0x04, 0x05, 0x06];
        let good_frame = make_frame_be16(&good_payload);

        // Concatenate bad + good into one response chunk so the retry finds the good frame
        let mut combined = bad_frame;
        combined.extend_from_slice(&good_frame);
        let mock = MockTransport::new(vec![combined]);
        let mut rx_buf = Vec::new();

        let result = read_frame(
            &mut rx_buf,
            &mock,
            extract_frame_abcd_be16,
            |_| true,
            FrameErrorRecovery::SkipAndRetry,
            "test",
        )
        .unwrap();
        assert_eq!(result, good_payload);
    }

    #[test]
    fn read_frame_accept_filter_skips_non_matching() {
        // First frame has payload starting with 0x01 (rejected), second with 0x02 (accepted)
        let rejected_payload = vec![0x01, 0xAA, 0xBB];
        let accepted_payload = vec![0x02, 0xCC, 0xDD];
        let frame1 = make_frame_be16(&rejected_payload);
        let frame2 = make_frame_be16(&accepted_payload);

        let mut combined = frame1;
        combined.extend_from_slice(&frame2);
        let mock = MockTransport::new(vec![combined]);
        let mut rx_buf = Vec::new();

        let result = read_frame(
            &mut rx_buf,
            &mock,
            extract_frame_abcd_be16,
            |p| !p.is_empty() && p[0] == 0x02,
            FrameErrorRecovery::Propagate,
            "test",
        )
        .unwrap();
        assert_eq!(result, accepted_payload);
    }

    #[test]
    fn read_frame_existing_data_in_rx_buf() {
        // Frame data is already in rx_buf before read_frame is called
        let payload = vec![0x01, 0x02, 0x03];
        let frame = make_frame_be16(&payload);
        let mock = MockTransport::new(vec![]); // no transport reads needed
        let mut rx_buf = frame;

        let result = read_frame(
            &mut rx_buf,
            &mock,
            extract_frame_abcd_be16,
            |_| true,
            FrameErrorRecovery::Propagate,
            "test",
        )
        .unwrap();
        assert_eq!(result, payload);
        assert!(rx_buf.is_empty());
    }
}
