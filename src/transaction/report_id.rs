//! 0x11
//!

use crate::{builder, device, exception, frame, Exception, Function};

pub const FUNCTION: crate::Function = crate::function::REPORT_SLAVE_ID;

pub struct Request<'a> {
    frame: frame::Frame<'a>,
}

impl Request<'_> {
    pub fn parse_from(frame: frame::Frame<'_>) -> Result<Request, Exception> {
        // report id has no payload bytes
        if frame.function() != FUNCTION {
            Err(exception::ILLEGAL_FUNCTION) // potentially should be panic'ing here?
        } else if frame.payload().len() == 0 {
            Ok(Request { frame })
        } else {
            Err(exception::ILLEGAL_DATA)
        }
    }

    pub fn device(&self) -> device::Device {
        self.frame.device()
    }

    pub fn function(&self) -> Function {
        self.frame.function()
    }

    pub fn build_response<'a>(
        &self,
        write_to: &'a mut [u8],
        device: device::Device,
        response_formatter: impl FnOnce(builder::Builder<'a, builder::AddData>) -> frame::Frame<'a>,
    ) -> frame::Frame<'a> {
        response_formatter(
            builder::build_frame(write_to)
                .for_device(device)
                .function(FUNCTION),
        )
    }
}

pub struct Response {
    // user defined data?
}
