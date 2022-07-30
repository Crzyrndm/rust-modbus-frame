//! # A simple modbus RTU library
//!
//! By default this library knows about Coils (1/5/15), Discrete Inputs(2), Holding Registers(3/6/16), and Input Registers(4) only
//! Users can extend this by replacing the encoder implementation
//!
//! ## Decode
//!
//! Takes in a slice of bytes, does basic validation (length/crc) then passes to the decoder
//! Decoder returns an enum (e.g. ReadHoldingRegisters(address, func, num_regs, &[regs], crc)) which the application can then act upon
//! Decoding doesn't require any copies to be made. Only references into the byte array
//!
//! ```rust
//! // TODO: decode example
//! ```
//!
//! ## Encode
//!
//! Encoding is done using the builder pattern. The builder is initialised with the scratch array, which it then fills in as
//! address, function, etc. are provided
//!
//! ```
//! use modbus_frames::{builder, Function};
//!
//! let mut buff = [0u8; 20];
//! let frame = builder::build_frame(&mut buff)
//!                 .for_address(1)
//!                 .function(Function(2))
//!                 .register(3)
//!                 .finalise();
//! assert_eq!(frame.raw_bytes(), [1, 2, 0, 3, 224, 25]);
//! assert_eq!(frame.payload(), [0, 3]);
//! ```

pub mod builder;
pub mod decoder;
pub mod exception;
pub mod frame;
pub mod function;

pub use exception::Exception;
pub use frame::Frame;
pub use function::Function;

pub fn calculate_crc16(bytes: &[u8]) -> u16 {
    crc16::State::<crc16::MODBUS>::calculate(bytes)
}

/// returns true if `bytes.len() > 4` and the last two bytes are the CRC16 of the preceding bytes
pub fn verify_crc16(bytes: &[u8]) -> bool {
    if bytes.len() < 4 {
        false
    } else {
        let frame = Frame::new(bytes);
        frame.calculate_crc().to_le_bytes() == frame.crc_bytes()
    }
}

pub trait PacketLen {
    fn packet_len(&self) -> u8;
    fn minimum_len() -> u8;
}

pub trait FixedLen: PacketLen {
    const LEN: u8;
}

impl<T: FixedLen> PacketLen for T {
    fn packet_len(&self) -> u8 {
        Self::LEN
    }

    fn minimum_len() -> u8 {
        Self::LEN
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
#[non_exhaustive] // new errors may be added later
pub enum Error {
    /// Valid message lengths are 4-256 bytes
    InvalidLength,
    /// CRC verification failed
    InvalidCrc,
    /// Decoding failed because the function code was unknown
    UnknownFunction,
    /// message size is invalid for the function code
    DecodeInvalidLength,
}

// std::error::Error trait obviously isn't available in no_std
// whould this implement any other error traits?

#[cfg(test)]
mod tests {
    use crate::{calculate_crc16, verify_crc16};

    #[test]
    fn crc_calculation() {
        // example request message bytes from https://www.simplymodbus.ca/FC06.htm
        let message = [0x11_u8, 0x06, 0x00, 0x01, 0x00, 0x03, 0x9A, 0x9B];
        let crc_idx = message.len() - 2;
        assert_eq!(
            calculate_crc16(&message[..crc_idx]),
            u16::from_le_bytes([0x9A, 0x9B]) // CRC is little endian
        );
        assert!(verify_crc16(&message));
    }
}
