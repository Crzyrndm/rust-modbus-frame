//! # Simple modbus RTU library
//!
//! By default this library knows about Coils (1/5/15), Discrete Inputs(2), Holding Registers(3/6/16), and Input Registers(4) only
//! Users can extend this by replacing the encoder implementation
//!
//! ## Decode
//!
//! Takes in a slice of bytes, does basic validation (length/crc) then passes to the decoder
//! Decoder returns an enum (e.g. ReadHoldingRegisters(address, func, num_regs, &[regs], crc)) which the application can then act upon
//! Decoding should not require any copies to be made. Only references into the byte array
//!
//! ## Encode
//!
//! Encoding is done using the builder pattern. The builder is initialised with the scratch array, which it then fills in as
//! address, function, etc. are provided

pub mod builder;
pub mod decoder;
pub mod exception;
pub mod frame;
pub mod function;

pub use exception::Exception;
pub use function::Function;

pub fn calculate_crc16(bytes: &[u8]) -> u16 {
    crc16::State::<crc16::MODBUS>::calculate(bytes)
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

#[derive(PartialEq, Debug, Clone)]
#[non_exhaustive] // new errors may be added later
pub enum Error {
    None,
    InvalidLength,
    InvalidCorrupt,
    InvalidEncoding,
    OtherAddress,
    WrongFunction,
}

// std::error::Error trait obviously isn't available in no_std
// whould this implement any other error traits?

#[cfg(test)]
mod tests {
    //
}
