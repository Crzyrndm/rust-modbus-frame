use byteorder::ByteOrder;

use crate::{builder, calculate_crc16, verify_crc16, Error, Exception, Function};

/// Frame provides functions to view a series of bytes in RTU format as a modbus data frame
/// `|address(1)|function(1)|payload(0..252)|crc16(2)`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<'b> {
    data: &'b [u8],
}

impl<'b> Frame<'b> {
    /// Creates a new frame without validation
    ///
    /// # UNCHECKED
    /// if `bytes.len() < 4` The created frame will be invalid and later operations are likely to panic.
    ///
    /// Prefer to use other methods to create Frame objects
    /// * from &[u8]: use Frame::try_from, invalid length or CRC will result in an error
    /// * frame::build_frame will construct a valid frame from various components in a reasonably ergonomic form
    ///
    /// This method is public to allow for potential external extensions
    pub fn new_unchecked(bytes: &'b [u8]) -> Self {
        Frame { data: bytes }
    }

    /// The address byte of the frame
    pub fn address(&self) -> u8 {
        self.data[0]
    }

    /// the function code of the frame
    pub fn function(&self) -> Function {
        Function(self.data[1])
    }

    /// calculate the expected CRC of the frame
    ///
    /// NOTE: if Self::new_unchecked was used to create this instance, there is a possibility this will not be equal to `self.crc()`
    pub fn calculate_crc(&self) -> u16 {
        let crc_idx = self.data.len() - 2;
        calculate_crc16(&self.data[..crc_idx])
    }

    /// All bytes between the address/function code and CRC
    pub fn payload(&self) -> &[u8] {
        let crc_idx = self.data.len() - 2;
        &self.data[2..crc_idx]
    }

    /// crc bytes as a u16
    pub fn crc(&self) -> u16 {
        byteorder::LittleEndian::read_u16(self.crc_bytes())
    }

    /// The crc bytes
    pub fn crc_bytes(&self) -> &[u8] {
        let crc_idx = self.data.len() - 2;
        &self.data[crc_idx..]
    }

    /// All of the bytes in the message (address, function, payload, crc)
    pub fn raw_bytes(&self) -> &[u8] {
        self.data
    }

    /// All of the bytes in the message (address, function, payload, crc)
    /// Consumes `Self` to tie the lifetime to the underlying array
    pub fn into_raw_bytes(self) -> &'b [u8] {
        self.data
    }

    /// Iterator returning the message bytes in RTU format
    pub fn rtu_bytes(&self) -> impl Iterator<Item = u8> + 'b {
        self.data.iter().copied()
    }

    /// Iterator returning the message bytes in ASCII format
    pub fn ascii_bytes(&self) /*-> impl Iterator<Item = u8> + 'b*/
    {
        todo!()
    }

    pub fn response_builder<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
    ) -> builder::Builder<'buff, builder::AddData> {
        builder::build_frame(response_buffer)
            .for_address(self.address())
            .function(self.function())
    }

    pub fn response_exception<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        exception: Exception,
    ) -> (Frame<'buff>, &'buff mut [u8]) {
        builder::build_frame(response_buffer)
            .for_address(self.address())
            .exception(self.function(), exception)
    }
}

impl<'b> TryFrom<&'b [u8]> for Frame<'b> {
    type Error = Error;

    fn try_from(bytes: &'b [u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 4 {
            Err(Self::Error::InvalidLength)
        } else if !verify_crc16(bytes) {
            Err(Self::Error::InvalidCrc)
        } else {
            Ok(Frame::new_unchecked(bytes))
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
        let frame = Frame::new_unchecked(&test_data);

        assert_eq!(frame.address(), 0);
        assert_eq!(frame.function(), Function(1));
        assert_eq!(frame.payload(), [2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(frame.raw_bytes(), test_data);
        assert_eq!(frame.raw_bytes(), frame.rtu_bytes().collect::<Vec<_>>());
        assert_eq!(frame.crc_bytes(), [116, 69]);
        assert_eq!(frame.crc_bytes(), frame.calculate_crc().to_le_bytes());
        assert_eq!(frame.crc(), frame.calculate_crc());
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
        assert_eq!(frame.calculate_crc().to_le_bytes(), frame.crc_bytes());
        // and since no copies were made, a view of the original bytes is available (excluding CRC)
        assert_eq!(frame.raw_bytes(), bytes);
    }
}
