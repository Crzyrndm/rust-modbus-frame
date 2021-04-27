//! 0x03

use core::convert::TryInto;

use crate::{exception, frame, iter::registers::Registers, Exception};

pub const FUNCTION: crate::Function = crate::function::READ_HOLDING_REGISTERS;

/** request payload
- starting register location (u16)
- number of registers to be read (u16)
*/
#[derive(Debug, PartialEq)]
pub struct Request<'b> {
    payload: &'b [u8],
}

impl<'b> Request<'b> {
    pub fn parse_from(frame: &'b frame::Frame<'b>) -> Result<Request<'b>, Option<Exception>> {
        // read registers request always has a 4 byte payload (address + length)
        if frame.function() != FUNCTION {
            Err(Some(exception::ILLEGAL_FUNCTION)) // potentially should be panic'ing here?
        } else if frame.payload().len() == 4 {
            let req = Request {
                payload: frame.payload(),
            };
            // max payload size is 256, less 4 for frame, less 1 for length = 251
            // split into 2-byte segments = 125
            const MAX_READ_COUNT: u16 = 125;
            if req.register_count() <= MAX_READ_COUNT {
                Ok(req)
            } else {
                Err(Some(exception::ILLEGAL_DATA))
            }
        } else {
            Err(None)
        }
    }

    pub fn address(&self) -> u16 {
        u16::from_be_bytes(self.payload[..2].try_into().unwrap())
    }

    pub fn register_count(&self) -> u16 {
        u16::from_be_bytes(self.payload[2..4].try_into().unwrap())
    }
}

/** response payload
- u8 specifying the number of bytes following
- array of u16 register values requested, starting from the specified address and incrementing
*/
#[derive(Debug, PartialEq)]
pub struct Response<'b> {
    payload: &'b [u8],
}

impl<'b> Response<'b> {
    pub fn parse_from(frame: &'b frame::Frame<'b>) -> Result<Response<'b>, Option<Exception>> {
        if frame.function().0 & 0x7F != FUNCTION.0 {
            // TODO: not quite what I want to respond with as this overlaps with the device sending this exception back
            Err(Some(exception::ILLEGAL_FUNCTION))
        } else if (frame.function().0 & 0x80) == 0x80 && !frame.payload().is_empty() {
            let exception_code = frame.payload()[0];
            Err(Some(Exception(exception_code)))
        } else {
            let valid = frame.payload().len() >= 3 // atleast 1 register in response
                        && frame.payload().len() == (frame.payload()[0] as usize + 1); // length byte == actual length
            if valid {
                Ok(Response {
                    payload: frame.payload(),
                })
            } else {
                // TODO: is this the correct response to use for "invalid format"? Provably not
                Err(None)
            }
        }
    }

    pub fn num_data_bytes(&self) -> u8 {
        self.payload[0]
    }

    pub fn register_count(&self) -> u16 {
        ((self.payload.len() - 1) / 2) as u16
    }

    pub fn registers(&self) -> crate::iter::registers::Registers {
        Registers::create(&self.payload[1..])
    }
}

#[cfg(test)]
mod tests {

    use crate::{exception, frame::Frame};

    use super::{Request, Response, FUNCTION};

    #[test]
    fn test_request_impl() {
        let payload = [0x00, FUNCTION.0, 0x45, 0x59, 0x00, 0x31];
        let frame = Frame::new(&payload[..]);
        let req = Request::parse_from(&frame).unwrap();
        assert_eq!(req.address(), 0x4559);
        assert_eq!(req.register_count(), 0x0031);

        // request count too high
        let payload = [0x00, FUNCTION.0, 0x45, 0x59, 0x00, 126];
        let frame = Frame::new(&payload[..]);
        assert_eq!(
            Request::parse_from(&frame),
            Err(Some(crate::exception::ILLEGAL_DATA))
        );

        // payload too short
        let payload = [0x00, FUNCTION.0, 0x45, 0x59, 0x00];
        let frame = Frame::new(&payload[..]);
        assert_eq!(Request::parse_from(&frame), Err(None));

        // payload too long
        let payload = [0x00, FUNCTION.0, 0x45, 0x59, 0x00, 0x00, 0x00];
        let frame = Frame::new(&payload[..]);
        assert_eq!(Request::parse_from(&frame), Err(None));

        // wrong function
        let payload = [0x00, FUNCTION.0 + 1, 0x45, 0x59, 0x00, 0x00, 0x00];
        let frame = Frame::new(&payload[..]);
        assert_eq!(
            Request::parse_from(&frame),
            Err(Some(exception::ILLEGAL_FUNCTION))
        );
    }

    #[test]
    fn test_response_impl() {
        let payload = [0x00, FUNCTION.0, 4, 0x59, 0x00, 0x31, 0x01];
        let frame = Frame::new(&payload[..]);
        let req = Response::parse_from(&frame).unwrap();
        assert_eq!(req.num_data_bytes(), 4);
        assert_eq!(req.register_count(), 2);
        let registers: Vec<_> = req.registers().collect();
        assert_eq!(registers, [0x5900, 0x3101]);

        // invalid function
        let payload = [0x00, FUNCTION.0 + 1, 4, 0x59, 0x00, 0x31, 0x01];
        let frame = Frame::new(&payload[..]);
        assert_eq!(
            Response::parse_from(&frame),
            Err(Some(exception::ILLEGAL_FUNCTION))
        );

        // exception reqponse
        let payload = [0x00, 0x80 | FUNCTION.0, exception::DEVICE_FAILURE.0];
        let frame = Frame::new(&payload[..]);
        assert_eq!(
            Response::parse_from(&frame),
            Err(Some(exception::DEVICE_FAILURE))
        );

        // length mismatch
        let payload = [0x00, FUNCTION.0, 4, 0x00, 0x00];
        let frame = Frame::new(&payload[..]);
        assert_eq!(Response::parse_from(&frame), Err(None));
    }
}
