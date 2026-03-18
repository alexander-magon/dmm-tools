use crate::error::{Error, Result};
use log::{debug, trace};

/// Header bytes for all messages.
pub const HEADER: [u8; 2] = [0xAB, 0xCD];

/// Minimum valid response length: header(2) + length(1) + payload(1) + checksum(2) = 6
const MIN_RESPONSE_LEN: usize = 6;

/// Expected payload length for a measurement response.
pub const MEASUREMENT_PAYLOAD_LEN: usize = 14;

/// Find a complete framed response in the buffer.
///
/// Scans for `AB CD` header, reads the length byte, validates the checksum,
/// and returns the payload bytes (excluding header, length, and checksum).
///
/// Returns `Ok(Some((payload, consumed)))` if a valid frame is found,
/// where `consumed` is how many bytes to drain from the buffer.
/// Returns `Ok(None)` if the buffer doesn't yet contain a complete frame.
/// Returns `Err` if a frame is found but the checksum is invalid.
pub fn extract_frame(buf: &[u8]) -> Result<Option<(Vec<u8>, usize)>> {
    // Scan for header
    let Some(start) = buf
        .windows(2)
        .position(|w| w == HEADER)
    else {
        return Ok(None);
    };

    let remaining = &buf[start..];
    if remaining.len() < MIN_RESPONSE_LEN {
        return Ok(None);
    }

    // Byte after header is the payload length
    let payload_len = remaining[2] as usize;
    let frame_len = 2 + 1 + payload_len + 2; // header + len_byte + payload + checksum

    if remaining.len() < frame_len {
        return Ok(None);
    }

    let frame = &remaining[..frame_len];
    trace!("protocol: raw frame: {:02X?}", frame);

    // Checksum: sum of all bytes except the last two
    let data_bytes = &frame[..frame_len - 2];
    let computed: u16 = data_bytes.iter().map(|&b| b as u16).sum();
    let received = u16::from_be_bytes([frame[frame_len - 2], frame[frame_len - 1]]);

    if computed != received {
        debug!(
            "protocol: checksum mismatch: computed={computed:#06x}, received={received:#06x}"
        );
        return Err(Error::ChecksumMismatch {
            expected: received,
            actual: computed,
        });
    }

    let payload = frame[3..3 + payload_len].to_vec();
    let consumed = start + frame_len;

    debug!("protocol: valid frame, payload_len={payload_len}, consumed={consumed}");
    Ok(Some((payload, consumed)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a valid frame from a payload.
    fn make_frame(payload: &[u8]) -> Vec<u8> {
        let mut frame = vec![0xAB, 0xCD, payload.len() as u8];
        frame.extend_from_slice(payload);
        let sum: u16 = frame.iter().map(|&b| b as u16).sum();
        frame.push((sum >> 8) as u8);
        frame.push((sum & 0xFF) as u8);
        frame
    }

    #[test]
    fn extract_valid_frame() {
        let payload = vec![0x01, 0x02, 0x03];
        let frame = make_frame(&payload);
        let result = extract_frame(&frame).unwrap().unwrap();
        assert_eq!(result.0, payload);
        assert_eq!(result.1, frame.len());
    }

    #[test]
    fn extract_with_leading_garbage() {
        let payload = vec![0x01, 0x02, 0x03];
        let frame = make_frame(&payload);
        let mut buf = vec![0xFF, 0xFE, 0xFD];
        buf.extend_from_slice(&frame);
        let result = extract_frame(&buf).unwrap().unwrap();
        assert_eq!(result.0, payload);
        assert_eq!(result.1, 3 + frame.len()); // garbage + frame
    }

    #[test]
    fn extract_incomplete() {
        let frame = vec![0xAB, 0xCD, 0x03, 0x01]; // incomplete
        assert!(extract_frame(&frame).unwrap().is_none());
    }

    #[test]
    fn extract_bad_checksum() {
        let mut frame = make_frame(&[0x01, 0x02, 0x03]);
        let last = frame.len() - 1;
        frame[last] ^= 0xFF; // corrupt checksum
        assert!(extract_frame(&frame).is_err());
    }

    #[test]
    fn extract_no_header() {
        let buf = vec![0x00, 0x01, 0x02, 0x03];
        assert!(extract_frame(&buf).unwrap().is_none());
    }
}
