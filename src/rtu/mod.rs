pub mod crc;
pub mod iterator;
pub mod timing;

pub use iterator::AsBytesIter;

use crate::{error, frame::Frame, Result};

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
        return error::Error::InvalidCorrupt; // invalid crc
    }
    return error::Error::None;
}

pub fn decode(bytes: &[u8]) -> Result<Frame> {
    match validate(bytes) {
        error::Error::None => {
            // crc bytes aren't part of the frame
            let data = &bytes[..(bytes.len() - 2)];
            Ok(unsafe { Frame::new_unchecked(data) })
        }
        other => Err(other),
    }
}

#[cfg(test)]
mod tests {
    use super::validate;
    use crate::{error, rtu::crc};

    #[test]
    fn test_device_validate_msg() {
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

                    assert!(error::Error::InvalidCorrupt == validate(msg));
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
