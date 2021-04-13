use crate::{
    ascii,
    device::Device,
    rtu::{self, crc},
    Function,
};

/// Frame provides functions to view a series of bytes in RTU format as a modbus data frame
#[derive(PartialEq, Debug, Clone)]
pub struct Frame<'b> {
    data: &'b [u8],
}

impl<'b> Frame<'b> {
    /// panics if bytes.len() < 2
    ///
    /// This should be used internally to constructs that enforce that the input generates the expected frame
    /// - from &[u8] -> use Frame::try_from, invalid length or CRC will result in an error
    /// - frame::build_frame will construct a valid frame from various components in a reasonably ergonomix form
    /// - method is public to allow for potential extensions
    pub fn new(bytes: &'b [u8]) -> Self {
        // smallest valid modbus data is just address/function. Anything smaller is invalid
        assert!(bytes.len() >= 2);
        Frame { data: bytes }
    }

    pub fn device(&self) -> Device {
        Device::new(self.data[0])
    }

    pub fn function(&self) -> Function {
        Function(self.data[1])
    }

    pub fn crc(&self) -> u16 {
        crc::calculate(self.data)
    }

    pub fn payload(&self) -> &[u8] {
        &self.data[2..]
    }

    pub fn raw_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn rtu_bytes(&self) -> rtu::AsBytesIter {
        rtu::AsBytesIter::new(self.clone())
    }

    pub fn ascii_bytes(&self) -> ascii::AsBytesIter {
        ascii::AsBytesIter::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::Frame;
    use crate::{device::Device, function, rtu, Function};

    #[test]
    fn test_frame_views() {
        let test_data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let frame = Frame::new(&test_data[..]);

        assert_eq!(frame.device(), Device::new(0));
        assert_eq!(frame.function(), Function(1));
        assert_eq!(frame.payload(), [2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(frame.crc().to_le_bytes(), [116, 69]);
    }

    #[test]
    fn test_decode_from_bytes() {
        // incoming bytes
        let bytes: &[u8] = &[0x11, 0x03, 0x00, 0x6B, 0x00, 0x03, 0x76, 0x87];
        // try_from checks that the length is within modbus allowances (4 <= len <= 255)
        // and that the crc is valid.
        // frame::Frame is a borrow of the slice providing named accesor functions  for the bytes within
        if let Ok(frame) = rtu::decode(bytes) {
            assert_eq!(frame.device(), Device::new(0x11));
            assert_eq!(frame.function(), function::READ_HOLDING_REGISTERS);
            assert_eq!(frame.payload(), [0x00, 0x6B, 0x00, 0x03]);
            assert_eq!(frame.crc().to_le_bytes(), [0x76, 0x87]);
            // and since no copies were made, a view of the original bytes is available (excluding CRC)
            assert_eq!(frame.raw_bytes(), &bytes[..(bytes.len() - 2)]);
        }
    }
}
