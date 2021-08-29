//! 0x06

use core::convert::TryInto;

use crate::{
    builder,
    device::Device,
    exception,
    frame::{self, Frame},
    iter::registers::Registers,
    Exception, Function,
};

pub const FUNCTION: crate::Function = crate::function::WRITE_HOLDING_REGISTER;

/** request payload
- register location (u16)
- register value (u16)
*/
#[derive(Debug, PartialEq)]
pub struct Request<'a> {
    frame: Frame<'a>,
}

impl Request<'_> {
    pub fn parse_from(frame: frame::Frame<'_>) -> Result<Request<'_>, Exception> {
        // read registers request always has a 4 byte payload (address + length)
        if frame.function() != FUNCTION {
            Err(exception::ILLEGAL_FUNCTION) // potentially should be panic'ing here?
        } else if frame.payload().len() == 4 {
            Ok(Request { frame })
        } else {
            Err(exception::ILLEGAL_DATA)
        }
    }

    pub fn device(&self) -> Device {
        self.frame.device()
    }

    pub fn function(&self) -> Function {
        self.frame.function()
    }

    pub fn register_location(&self) -> u16 {
        let mut address_bytes = [0u8; 2];
        address_bytes.copy_from_slice(&self.frame.payload()[..2]);
        u16::from_be_bytes(address_bytes)
    }

    pub fn register_value(&self) -> u16 {
        let mut address_bytes = [0u8; 2];
        address_bytes.copy_from_slice(&self.frame.payload()[2..4]);
        u16::from_be_bytes(address_bytes)
    }

    /// fn here for compatibility with write_multiple
    pub fn registers(&self) -> Registers<'_> {
        Registers::create(&self.frame.payload()[2..])
    }

    pub fn build_response<'a>(&self, write_to: &'a mut [u8], device: Device) -> frame::Frame<'a> {
        assert!(write_to.len() >= 6); // TODO should this be a result?
        builder::build_frame(write_to)
            .for_device(device)
            .function(FUNCTION)
            .register(self.register_location())
            .register(self.register_value())
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
    pub fn parse_from(frame: frame::Frame<'b>) -> Result<Response<'b>, Exception> {
        Request::parse_from(frame).map(|req| Response { req })
    }

    pub fn address(&self) -> u16 {
        self.req.register_location()
    }

    pub fn value(&self) -> u16 {
        self.req.register_value()
    }
}
