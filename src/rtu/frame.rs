use crate::{
    builder::{Builder, Initial},
    device::Device,
    error,
    rtu::crc,
    Function, Result,
};
use core::convert::{TryFrom, TryInto};

use super::RTU;

/// Frame provides functions to view a series of bytes as a modbus data frame
#[derive(PartialEq, Debug, Clone)]
pub struct Frame<'b> {
    data: &'b [u8],
}

impl<'b> Frame<'b> {
    /// Assumes validation occurs prior to call as no guarantees are checked here
    /// in particular, this will lead to panics if bytes.len() is less than 2
    /// requirements for this to be safe
    /// - buffer size is in the inclusive range [4:255]
    /// - the last two bytes are the correct modbus crc (crc::calculate()) for the preceding bytes
    ///
    /// The safe alternative depend on data source.
    /// - &[u8] -> use Frame::try_from, invalid length or CRC will result in an error
    /// - frame::build_frame will construct a valid frame from various components in a reasonably ergonomix form
    ///
    /// new_unchecked is primarily intended for use in implementing (Try)Into for custom types where TryFrom is unnecesary
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
        let data_len = self.data.len() - 2;
        u16::from_le_bytes(self.data[data_len..].try_into().unwrap())
    }

    pub fn payload(&self) -> &[u8] {
        let data_len = self.data.len() - 2;
        &self.data[2..data_len]
    }

    pub fn raw_bytes(&self) -> &[u8] {
        &self.data
    }
}

/// a Frame has minimal requirements:
/// - be atleast 4 bytes. ANything less cannot exist
/// - have a valid CRC as the final 2 bytes
///
/// Commands and responses may impose tighter restrictions, but the raw frame is left open
/// (e.g. all of the standard frames require the length be <= 255)
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
            Self::Error::None => Ok(Frame { data: bytes }),
            other => Err(other),
        }
    }
}

impl<'b> TryFrom<&'b mut [u8]> for Frame<'b> {
    type Error = error::Error;

    fn try_from(bytes: &'b mut [u8]) -> Result<Self> {
        match validate(bytes) {
            Self::Error::None => Ok(Frame { data: bytes }),
            other => Err(other),
        }
    }
}

pub fn build<'b>(buffer: &'b mut [u8]) -> Builder<Frame<'b>, RTU, Initial> {
    Builder::from(RTU { buffer, idx: 0 })
}

#[cfg(test)]
mod tests {
    use crate::{
        device::Device,
        rtu::{
            crc,
            frame::{build, Frame},
        },
        Function,
    };

    #[test]
    fn test_frame_views() {
        let test_data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        // this frame is *definitely* invalid (bad crc)
        // only used because this is a dead simple test asserting the accesor functions work
        // getting the crc right only obscures the purpose
        let frame = unsafe { Frame::new_unchecked(&test_data[..]) };

        assert_eq!(frame.device(), Device::new(0));
        assert_eq!(frame.function(), Function(1));
        assert_eq!(frame.payload(), [2, 3, 4, 5, 6, 7]);
        assert_eq!(frame.crc().to_le_bytes(), [8, 9]);
    }

    #[test]
    fn test_builder() {
        let mut buff = [0u8; 20];
        // address state
        let frame = build(&mut buff);
        assert_eq!(0, frame.bytes_consumed());
        assert_eq!(20, frame.bytes_remaining());
        // function state
        let frame = frame.for_device(&Device::new(123));
        assert_eq!(1, frame.bytes_consumed());
        assert_eq!(19, frame.bytes_remaining());
        // data state
        let frame = frame.function(Function(213));
        assert_eq!(2, frame.bytes_consumed());
        assert_eq!(18, frame.bytes_remaining());

        let frame = frame.byte(1).register(4).bytes(&[2, 3]).registers(&[5, 6]);
        assert_eq!(11, frame.bytes_consumed());
        assert_eq!(9, frame.bytes_remaining());
        // as frame
        let frame = frame.finalise();
        assert_eq!(13, frame.raw_bytes().len());
        assert_eq!(Device::new(123), frame.device());
        assert_eq!(Function(213), frame.function());
        assert_eq!([1, 0, 4, 2, 3, 0, 5, 0, 6], frame.payload());

        let frame_crc = frame.crc();
        let crc = crc::calculate(&buff[..11]);
        assert_eq!(crc, frame_crc);
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
}
