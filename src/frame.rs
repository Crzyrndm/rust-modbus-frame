use crate::{calculate_crc16, Function};

/// Frame provides functions to view a series of bytes in RTU format as a modbus data frame
#[derive(PartialEq, Debug, Clone)]
pub struct Frame<'b> {
    data: &'b [u8],
}

impl<'b> Frame<'b> {
    /// panics if bytes.len() < 4
    ///
    /// This should be used internally to constructs that enforce that the input generates the expected frame
    /// - from &[u8] -> use Frame::try_from, invalid length or CRC will result in an error
    /// - frame::build_frame will construct a valid frame from various components in a reasonably ergonomic form
    /// - method is public to allow for potential extensions
    pub fn new(bytes: &'b [u8]) -> Self {
        // smallest valid modbus data is just address/function + CRC. Anything smaller is invalid
        assert!(bytes.len() >= 4);
        Frame { data: bytes }
    }

    pub fn address(&self) -> u8 {
        self.data[0]
    }

    pub fn function(&self) -> Function {
        Function(self.data[1])
    }

    pub fn calulate_crc(&self) -> u16 {
        let crc_idx = self.data.len() - 2;
        calculate_crc16(&self.data[..crc_idx])
    }

    pub fn payload(&self) -> &[u8] {
        let crc_idx = self.data.len() - 2;
        &self.data[2..crc_idx]
    }

    pub fn crc_bytes(&self) -> &[u8] {
        let crc_idx = self.data.len() - 2;
        &self.data[crc_idx..]
    }

    pub fn raw_bytes(&self) -> &[u8] {
        self.data
    }

    pub fn into_raw_bytes(self) -> &'b [u8] {
        self.data
    }

    pub fn rtu_bytes(&self) -> impl Iterator<Item = u8> + 'b {
        self.data.iter().copied()
    }

    pub fn ascii_bytes(&self) /*-> impl Iterator<Item = u8> + 'b*/
    {
        todo!()
    }
}

impl<'b> TryFrom<&'b [u8]> for Frame<'b> {
    type Error = ();

    fn try_from(value: &'b [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::new(value);
        if frame.calulate_crc().to_le_bytes() == frame.crc_bytes() {
            Ok(frame)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Frame;
    use crate::{function, Function};

    #[test]
    fn test_frame_views() {
        let test_data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 116, 69];
        let frame = Frame::new(&test_data[..]);

        assert_eq!(frame.address(), 0);
        assert_eq!(frame.function(), Function(1));
        assert_eq!(frame.payload(), [2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(frame.calulate_crc().to_le_bytes(), [116, 69]);
    }

    #[test]
    fn test_decode_from_bytes() {
        // incoming bytes
        let bytes: &[u8] = &[0x11, 0x03, 0x00, 0x6B, 0x00, 0x03, 0x76, 0x87];
        // try_from checks that the length is within modbus allowances (4 <= len <= 255)
        // and that the crc is valid.
        // frame::Frame is a borrow of the slice providing named accesor functions  for the bytes within
        let frame = Frame::try_from(bytes).unwrap();
        assert_eq!(frame.address(), 0x11);
        assert_eq!(frame.function(), function::READ_HOLDING_REGISTERS);
        assert_eq!(frame.payload(), [0x00, 0x6B, 0x00, 0x03]);
        assert_eq!(frame.calulate_crc().to_le_bytes(), frame.crc_bytes());
        // and since no copies were made, a view of the original bytes is available (excluding CRC)
        assert_eq!(frame.raw_bytes(), bytes);
    }
}
