//! 0x06

use core::convert::TryInto;

use crate::{builder, device::Device, exception, frame, iter::registers::Registers, Exception};

pub const FUNCTION: crate::Function = crate::function::WRITE_HOLDING_REGISTER;

/** request payload
- register location (u16)
- register value (u16)
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
        } else if frame.payload().len() == 4 {
            Ok(Request {
                payload: frame.payload(),
            })
        } else {
            Err(exception::ILLEGAL_DATA)
        }
    }

    pub fn address(&self) -> u16 {
        u16::from_be_bytes(self.payload[..2].try_into().unwrap())
    }

    pub fn value(&self) -> u16 {
        u16::from_be_bytes(self.payload[2..4].try_into().unwrap())
    }

    /// fn here for compatibility with write_multiple
    pub fn data_byte_count(&self) -> u8 {
        2
    }

    /// fn here for compatibility with write_multiple
    pub fn registers(&self) -> Registers<'b> {
        Registers::create(&self.payload[2..])
    }

    pub fn build_response(&self, write_to: &'b mut [u8], device: Device) -> frame::Frame<'b> {
        assert!(write_to.len() >= 6); // TODO should this be a result?
        builder::build_frame(write_to)
            .for_device(device)
            .function(FUNCTION)
            .register(self.address())
            .register(self.value())
            .finalise()
    }
}

/** response payload
- address
- value
*/
#[derive(Debug, PartialEq)]
pub struct Response<'b> {
    // response just duplicates the request
    req: Request<'b>,
}

impl<'b> Response<'b> {
    pub fn parse_from(frame: &'b frame::Frame<'b>) -> Result<Response<'b>, Exception> {
        Request::parse_from(frame).map(|req| Response { req })
    }

    pub fn address(&self) -> u16 {
        self.req.address()
    }

    pub fn value(&self) -> u16 {
        self.req.value()
    }
}
