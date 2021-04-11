use crate::{
    ascii,
    device::Device,
    error,
    rtu::{self, crc},
    Function, Result,
};
use core::convert::TryFrom;

/// Frame provides functions to view a series of bytes as a modbus data frame
#[derive(PartialEq, Debug, Clone)]
pub struct Frame<'b> {
    data: &'b [u8],
}

impl<'b> Frame<'b> {
    /// Assumes validation occurs prior to call as no guarantees are checked here
    /// in particular, this will lead to panics if bytes.len() is less than 2
    /// requirements for this to be safe
    /// - buffer size is >= 2
    ///
    /// The safe alternative depend on data source.
    /// - &[u8] -> use Frame::try_from, invalid length or CRC will result in an error
    /// - frame::build_frame will construct a valid frame from various components in a reasonably ergonomix form
    ///
    /// new_unchecked is primarily intended for use in implementing (Try)From and the like
    pub unsafe fn new_unchecked(bytes: &'b [u8]) -> Self {
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

/// a Frame has minimal requirements:
/// - be atleast 4 bytes. Anything less cannot exist
/// - have a valid CRC as the final 2 bytes
///
/// Commands and responses may impose tighter restrictions, but the raw frame is left open
/// (e.g. all of the standard frames require the length be <= 256)
pub fn validate(bytes: &[u8]) -> error::Error {
    if bytes.len() < 4 {
        // incomplete / invalid data packet
        return error::Error::InvalidLength;
    }
    let data_len = bytes.len() - 2;
    let msg_crc = crc::calculate(&bytes[0..data_len]);
    if bytes[data_len..] != msg_crc.to_le_bytes() {
        return error::Error::InvalidCrC; // invalid crc
    }
    return error::Error::None;
}

impl<'b> TryFrom<&'b [u8]> for Frame<'b> {
    type Error = error::Error;

    fn try_from(bytes: &'b [u8]) -> Result<Self> {
        match validate(bytes) {
            Self::Error::None => {
                // crc bytes aren't part of the frame
                let data = &bytes[..(bytes.len() - 2)];
                Ok(Frame { data })
            }
            other => Err(other),
        }
    }
}

impl<'b> TryFrom<&'b mut [u8]> for Frame<'b> {
    type Error = error::Error;

    fn try_from(bytes: &'b mut [u8]) -> Result<Self> {
        let bytes: &[u8] = bytes; // mutable conversion is only provided as a convenience
        Frame::try_from(bytes)
    }
}

#[cfg(test)]
mod tests {
    use core::convert::TryFrom;

    use super::Frame;
    use crate::{device::Device, function, rtu::crc, Function};

    #[test]
    fn test_frame_views() {
        let test_data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let frame = unsafe { Frame::new_unchecked(&test_data[..]) };

        assert_eq!(frame.device(), Device::new(0));
        assert_eq!(frame.function(), Function(1));
        assert_eq!(frame.payload(), [2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(frame.crc().to_le_bytes(), [116, 69]);
    }

    #[test]
    fn test_device_validate_msg() {
        use super::validate;
        use crate::error;

        const MAX_LEN: usize = 300;
        const TEST_ADDRESS: u8 = 2;

        let mut msg_buffer = [0u8; MAX_LEN];
        for len in 0..MAX_LEN {
            let msg = &mut msg_buffer[0..len];
            match len {
                0..=3 => {
                    // too short, no point doing address/crc shenanigans
                    assert!(error::Error::InvalidLength == validate(msg));
                }
                _ => {
                    msg[0] = TEST_ADDRESS; // valid address
                    msg[1] = len as u8; // function cade == len
                    msg[2..] // fill the rest with incrementing numbers
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, v)| *v = i as u8);

                    assert!(error::Error::InvalidCrC == validate(msg));
                    // calculate actual crc
                    let data_len = msg.len() - 2;
                    let msg_crc_bytes = crc::calculate(&msg[0..data_len]).to_le_bytes();
                    msg[data_len] = msg_crc_bytes[0];
                    msg[data_len + 1] = msg_crc_bytes[1];
                    assert!(error::Error::None == validate(msg));
                }
            }
        }
    }

    #[test]
    fn device_try_from_bytes() {
        // incoming bytes
        let bytes: &[u8] = &[0x11, 0x03, 0x00, 0x6B, 0x00, 0x03, 0x76, 0x87];
        // try_from checks that the length is within modbus allowances (4 <= len <= 255)
        // and that the crc is valid.
        // frame::Frame is a borrow of the slice providing named accesor functions  for the bytes within
        if let Ok(frame) = Frame::try_from(bytes) {
            assert_eq!(frame.device(), Device::new(0x11));
            assert_eq!(frame.function(), function::READ_HOLDING_REGISTERS);
            assert_eq!(frame.payload(), [0x00, 0x6B, 0x00, 0x03]);
            assert_eq!(frame.crc().to_le_bytes(), [0x76, 0x87]);
            // and since no copies were made, a view of the original bytes is available (excluding CRC)
            assert_eq!(frame.raw_bytes(), &bytes[..(bytes.len() - 2)]);
        }
    }
}
