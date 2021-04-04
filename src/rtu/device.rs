use core::convert::TryFrom;

use crate::rtu::{self, frame::Frame};

#[derive(PartialEq, Debug, Clone)]
pub struct Device {
    adr: u8,
}

impl Device {
    pub fn new(address: u8) -> Self {
        Device { adr: address }
    }

    pub fn address(&self) -> u8 {
        self.adr
    }

    pub fn decode<'b>(&self, bytes: &'b [u8]) -> Result<Frame<'b>, rtu::Errors> {
        match Frame::try_from(bytes) {
            Ok(frame) => {
                if frame.address() == self.address() {
                    Ok(frame)
                } else {
                    Err(rtu::Errors::OtherAddress)
                }
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::convert::TryFrom;

    use super::{rtu::Errors, Device, Frame};
    use crate::rtu::crc;

    #[test]
    fn test_new_device() {
        for address in 1..255 {
            let device_1 = Device::new(address);
            assert_eq!(device_1.adr, address);
            assert_eq!(device_1.address(), address);
        }
    }

    #[test]
    fn test_device_decode_frame() {
        let test_device = Device::new(2);

        const MAX_LEN: usize = 300;
        let mut msg_buffer = [0u8; MAX_LEN];
        for len in 0..MAX_LEN {
            let msg = &mut msg_buffer[0..len];
            match len {
                0..=3 => {
                    // too short, no point doing address/crc shenanigans
                    assert!(Err(Errors::TooShort) == test_device.decode(msg));
                }
                4..=255 => {
                    msg[1] = len as u8; // function cade == len
                    msg[2..] // fill the rest with incrementing numbers
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, v)| *v = i as u8);

                    // crc is not calculated, so invalid
                    assert!(Err(Errors::InvalidCrC) == test_device.decode(msg));
                    // update the crc but with the wrong address
                    msg[0] = !test_device.address(); // bitflip address to get failing address
                    let data_len = msg.len() - 2;
                    let msg_crc_bytes = crc::calculate(&msg[0..data_len]).to_le_bytes();
                    msg[data_len] = msg_crc_bytes[0];
                    msg[data_len + 1] = msg_crc_bytes[1];
                    assert!(Err(Errors::OtherAddress) == test_device.decode(msg));
                    // now with the real address
                    msg[0] = test_device.address();
                    let msg_crc_bytes = crc::calculate(&msg[0..data_len]).to_le_bytes();
                    msg[data_len] = msg_crc_bytes[0];
                    msg[data_len + 1] = msg_crc_bytes[1];
                    let success = test_device.decode(msg);
                    assert!(success.is_ok());
                    assert!(Frame::try_from(&msg[..]) == success);
                }
                _ => {
                    // too long, no point doing address/crc shenanigans
                    assert!(Err(Errors::TooLong) == test_device.decode(msg));
                }
            }
        }
    }
}
