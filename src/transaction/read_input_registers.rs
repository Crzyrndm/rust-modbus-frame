//! 0x04

use crate::{
    builder::{self, AddData, Builder},
    device::Device,
    exception,
    frame::{self, Frame},
    iter::registers::Registers,
    Exception, Function,
};

pub const FUNCTION: crate::Function = crate::function::READ_INPUT_REGISTERS;

/** request payload
- starting register location (u16)
- number of registers to be read (u16)
*/
#[derive(Debug, PartialEq)]
pub struct Request<'b> {
    frame: Frame<'b>,
}

impl Request<'_> {
    pub fn parse_from<'b>(frame: frame::Frame<'b>) -> Result<Request<'b>, Exception> {
        // read registers request always has a 4 byte payload (address + length)
        if frame.function() != FUNCTION {
            Err(exception::ILLEGAL_FUNCTION)
        } else if frame.payload().len() == 4 {
            let req = Request { frame };
            // max payload size is 256, less 4 for frame, less 1 for length = 251
            // split into 2-byte segments = 125
            const MAX_READ_COUNT: u16 = 125;
            if req.register_count() <= MAX_READ_COUNT {
                Ok(req)
            } else {
                Err(exception::ILLEGAL_DATA)
            }
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

    pub fn first_register(&self) -> u16 {
        let mut address_bytes = [0u8; 2];
        address_bytes.copy_from_slice(&self.frame.payload()[..2]);
        u16::from_be_bytes(address_bytes)
    }

    pub fn register_count(&self) -> u16 {
        let mut address_bytes = [0u8; 2];
        address_bytes.copy_from_slice(&self.frame.payload()[2..4]);
        u16::from_be_bytes(address_bytes)
    }

    pub fn register_range(&self) -> core::ops::Range<u16> {
        let last_reg_address = self.first_register() + self.register_count();
        self.first_register()..last_reg_address
    }

    pub fn build_response_from_regs<'a, I: Iterator<Item = u16>>(
        &self,
        write_to: &'a mut [u8],
        device: Device,
        registers: I,
    ) -> Frame<'a> {
        self.build_response_with(write_to, device, |builder| builder.registers(registers))
    }

    pub fn build_response_with<'a, F>(
        &self,
        write_to: &'a mut [u8],
        device: Device,
        fill_regs: F,
    ) -> Frame<'a>
    where
        F: FnOnce(Builder<AddData>) -> Builder<AddData>,
    {
        let data_len_bytes = 2 * self.register_count() as u8;
        assert!(write_to.len() >= 3 + data_len_bytes as usize); // TODO should this be a result?
        let mut builder = builder::build_frame(write_to)
            .for_device(device)
            .function(FUNCTION)
            .byte(data_len_bytes);
        builder = fill_regs(builder);
        assert_eq!(builder.bytes_consumed(), 3 + data_len_bytes as usize);
        builder.finalise()
    }
}

/** response payload
- u8 specifying the number of bytes following
- array of u16 register values requested, starting from the specified address and incrementing
*/
#[derive(Debug, PartialEq)]
pub struct Response<'b> {
    frame: frame::Frame<'b>,
}

impl<'b> Response<'b> {
    pub fn parse_from(frame: frame::Frame<'b>) -> Result<Response<'b>, Exception> {
        if frame.function().0 & 0x7F != FUNCTION.0 {
            Err(exception::ILLEGAL_FUNCTION)
        } else if (frame.function().0 & 0x80) == 0x80 && !frame.payload().is_empty() {
            let exception_code = frame.payload()[0];
            Err(Exception(exception_code))
        } else {
            let valid = frame.payload().len() >= 3 // atleast 1 register in response
                        && frame.payload().len() == (frame.payload()[0] as usize + 1); // length byte == actual length
            if valid {
                Ok(Response { frame })
            } else {
                Err(exception::ILLEGAL_DATA)
            }
        }
    }

    pub fn num_data_bytes(&self) -> u8 {
        self.frame.payload()[0]
    }

    pub fn register_count(&self) -> u16 {
        ((self.frame.payload().len() - 1) / 2) as u16
    }

    pub fn registers(&self) -> crate::iter::registers::Registers {
        Registers::create(&self.frame.payload()[1..])
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
        let req = Request::parse_from(frame).unwrap();
        assert_eq!(req.first_register(), 0x4559);
        assert_eq!(req.register_count(), 0x0031);

        // request count too high
        let payload = [0x00, FUNCTION.0, 0x45, 0x59, 0x00, 126];
        let frame = Frame::new(&payload[..]);
        assert_eq!(
            Request::parse_from(frame),
            Err(crate::exception::ILLEGAL_DATA)
        );

        // payload too short
        let payload = [0x00, FUNCTION.0, 0x45, 0x59, 0x00];
        let frame = Frame::new(&payload[..]);
        assert_eq!(Request::parse_from(frame), Err(exception::ILLEGAL_DATA));

        // payload too long
        let payload = [0x00, FUNCTION.0, 0x45, 0x59, 0x00, 0x00, 0x00];
        let frame = Frame::new(&payload[..]);
        assert_eq!(Request::parse_from(frame), Err(exception::ILLEGAL_DATA));

        // wrong function
        let payload = [0x00, FUNCTION.0 + 1, 0x45, 0x59, 0x00, 0x00, 0x00];
        let frame = Frame::new(&payload[..]);
        assert_eq!(Request::parse_from(frame), Err(exception::ILLEGAL_FUNCTION));
    }

    #[test]
    fn test_response_impl() {
        let payload = [0x00, FUNCTION.0, 4, 0x59, 0x00, 0x31, 0x01];
        let frame = Frame::new(&payload[..]);
        let req = Response::parse_from(frame).unwrap();
        assert_eq!(req.num_data_bytes(), 4);
        assert_eq!(req.register_count(), 2);
        let registers: Vec<_> = req.registers().collect();
        assert_eq!(registers, [0x5900, 0x3101]);

        // invalid function
        let payload = [0x00, FUNCTION.0 + 1, 4, 0x59, 0x00, 0x31, 0x01];
        let frame = Frame::new(&payload[..]);
        assert_eq!(
            Response::parse_from(frame),
            Err(exception::ILLEGAL_FUNCTION)
        );

        // exception reqponse
        let payload = [0x00, 0x80 | FUNCTION.0, exception::DEVICE_FAILURE.0];
        let frame = Frame::new(&payload[..]);
        assert_eq!(Response::parse_from(frame), Err(exception::DEVICE_FAILURE));

        // length mismatch
        let payload = [0x00, FUNCTION.0, 4, 0x00, 0x00];
        let frame = Frame::new(&payload[..]);
        assert_eq!(Response::parse_from(frame), Err(exception::ILLEGAL_DATA));
    }

    #[test]
    fn test_request_build_response() {
        let payload = [0x00, FUNCTION.0, 0x45, 0x59, 0x00, 0x0A];
        let frame = Frame::new(&payload[..]);
        let req = Request::parse_from(frame.clone()).unwrap();

        let mut response_buffer = [0; 30];
        let regs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let response = req.build_response_from_regs(
            &mut response_buffer,
            frame.device(),
            regs.iter().copied(),
        );
        assert_eq!(
            &[
                0, FUNCTION.0, 20, 0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 54,
                43
            ][..],
            response.rtu_bytes().collect::<Vec<_>>()
        );

        let mut response_buffer = [0; 30];
        let response = req.build_response_with(&mut response_buffer, frame.device(), |builder| {
            builder
                .register(0)
                .register(1)
                .register(2)
                .register(3)
                .register(4)
                .register(5)
                .register(6)
                .register(7)
                .register(8)
                .register(9)
        });
        assert_eq!(
            &[
                0, FUNCTION.0, 20, 0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 54,
                43
            ][..],
            response.rtu_bytes().collect::<Vec<_>>()
        );
    }
}
