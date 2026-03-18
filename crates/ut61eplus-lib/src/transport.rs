use crate::error::Result;

/// Abstraction over HID transport for testability.
pub trait Transport {
    /// Write data to the device (interrupt OUT report).
    fn write(&self, data: &[u8]) -> Result<()>;

    /// Read data from the device (interrupt IN report).
    /// Returns the number of bytes read, or 0 on timeout.
    fn read_timeout(&self, buf: &mut [u8], timeout_ms: i32) -> Result<usize>;

    /// Send a HID feature report.
    fn send_feature_report(&self, data: &[u8]) -> Result<()>;
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::cell::RefCell;

    /// A mock transport that replays pre-recorded responses.
    pub struct MockTransport {
        responses: RefCell<Vec<Vec<u8>>>,
        pub written: RefCell<Vec<Vec<u8>>>,
        pub feature_reports: RefCell<Vec<Vec<u8>>>,
    }

    impl MockTransport {
        pub fn new(responses: Vec<Vec<u8>>) -> Self {
            Self {
                responses: RefCell::new(responses),
                written: RefCell::new(Vec::new()),
                feature_reports: RefCell::new(Vec::new()),
            }
        }
    }

    impl Transport for MockTransport {
        fn write(&self, data: &[u8]) -> Result<()> {
            self.written.borrow_mut().push(data.to_vec());
            Ok(())
        }

        fn read_timeout(&self, buf: &mut [u8], _timeout_ms: i32) -> Result<usize> {
            let mut responses = self.responses.borrow_mut();
            if responses.is_empty() {
                return Ok(0);
            }
            let response = responses.remove(0);
            let len = response.len().min(buf.len());
            buf[..len].copy_from_slice(&response[..len]);
            Ok(len)
        }

        fn send_feature_report(&self, data: &[u8]) -> Result<()> {
            self.feature_reports.borrow_mut().push(data.to_vec());
            Ok(())
        }
    }
}
