//! 0x10

use core::convert::TryInto;

use crate::{builder, device::Device, exception, frame, iter::registers::Registers, Exception};

pub const FUNCTION: crate::Function = crate::function::WRITE_MULTIPLE_HOLDING_REGISTERS;

/** request payload
- starting register location (u16)
- number of registers to be written (u16)
- number of data bytes (u8)
- data arr &\[u16\]
*/
#[derive(Debug, PartialEq)]
pub struct Request<'b> {
    payload: &'b [u8],
}

impl<'b> Request<'b> {
    pub fn parse_from(frame: &'b frame::Frame<'b>) -> Result<Request<'b>, Exception> {
        // read registers request always has a 4 byte payload (address + length)
        if frame.function() != FUNCTION {
            Err(exception::ILLEGAL_FUNCTION) // potentially should be panic'ing here?
        } else if frame.payload().len() >= 5 {
            let req = Request {
                payload: frame.payload(),
            };
            // max payload size is 256, less 4 for frame, less 5 for header = 247
            // split into 2-byte segments = 123
            const MAX_WRITE_COUNT: u16 = 123;
            if req.register_count() <= MAX_WRITE_COUNT
                // check num registers and num data bytes match up
                && req.register_count() * 2 == req.data_byte_count() as u16
                && frame.payload().len() - 5 == req.data_byte_count() as usize
            {
                Ok(req)
            } else {
                Err(exception::ILLEGAL_DATA)
            }
        } else {
            Err(exception::ILLEGAL_DATA)
        }
    }

    pub fn address(&self) -> u16 {
        u16::from_be_bytes(self.payload[..2].try_into().unwrap())
    }

    pub fn register_count(&self) -> u16 {
        u16::from_be_bytes(self.payload[2..4].try_into().unwrap())
    }

    pub fn data_byte_count(&self) -> u8 {
        self.payload[4]
    }

    pub fn registers(&self) -> Registers<'b> {
        Registers::create(&self.payload[5..])
    }

    pub fn build_response(&self, write_to: &'b mut [u8], device: Device) -> frame::Frame<'b> {
        assert!(write_to.len() >= 6); // TODO should this be a result?
        builder::build_frame(write_to)
            .for_device(device)
            .function(FUNCTION)
            .register(self.address())
            .register(self.register_count())
            .finalise()
    }
}

/** response payload
- starting address
- num regs written
*/
#[derive(Debug, PartialEq)]
pub struct Response<'b> {
    payload: &'b [u8],
}

impl<'b> Response<'b> {
    pub fn parse_from(frame: &'b frame::Frame<'b>) -> Result<Response<'b>, Exception> {
        // if function code doesn't match (7 bit, ignoring the exception bit)
        if frame.function().0 & 0x7F != FUNCTION.0 {
            Err(exception::ILLEGAL_FUNCTION) // panic?
        } else if (frame.function().0 & 0x80) == 0x80 && !frame.payload().is_empty() {
            let exception_code = frame.payload()[0];
            Err(Exception(exception_code))
        } else {
            let valid = frame.payload().len() == 4; // atleast 1 register in response
            if valid {
                Ok(Response {
                    payload: frame.payload(),
                })
            } else {
                Err(exception::ILLEGAL_DATA)
            }
        }
    }

    pub fn address(&self) -> u16 {
        u16::from_be_bytes(self.payload[..2].try_into().unwrap())
    }

    pub fn register_count(&self) -> u16 {
        u16::from_be_bytes(self.payload[2..4].try_into().unwrap())
    }
}
