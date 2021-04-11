use crate::{builder, device, error, frame, function, Result};
use core::convert::TryFrom;

#[derive(PartialEq, Debug, Clone)]
pub struct ReadRegisterCommand<'b> {
    frame: frame::Frame<'b>,
}

impl<'b> ReadRegisterCommand<'b> {
    pub fn new(
        buffer: &'b mut [u8],
        device: &device::Device,
        start_reg: u16,
        num_regs: u16,
    ) -> ReadRegisterCommand<'b> {
        assert!(buffer.len() >= 8); // need at least 8 bytes to format the command into

        let frame = builder::build_frame(buffer)
            .for_device(device)
            .function(function::READ_HOLDING_REGISTERS)
            .register(start_reg)
            .register(num_regs)
            .finalise();
        ReadRegisterCommand { frame }
    }

    pub fn start_register(&self) -> u16 {
        let bytes = [self.frame.raw_bytes()[2], self.frame.raw_bytes()[3]];
        u16::from_be_bytes(bytes)
    }

    pub fn register_count(&self) -> u16 {
        let bytes = [self.frame.raw_bytes()[4], self.frame.raw_bytes()[5]];
        u16::from_be_bytes(bytes)
    }

    pub fn end_register(&self) -> u16 {
        self.start_register() + self.register_count()
    }
}

impl<'b> TryFrom<&'b [u8]> for ReadRegisterCommand<'b> {
    type Error = error::Error;

    fn try_from(value: &'b [u8]) -> Result<Self> {
        if value.len() != 8 {
            return Err(Self::Error::InvalidLength);
        }
        if value[1] != function::READ_HOLDING_REGISTERS.0 {
            return Err(Self::Error::WrongFunction);
        }
        frame::Frame::try_from(value).map(|frame| ReadRegisterCommand { frame })
    }
}

impl<'b> TryFrom<frame::Frame<'b>> for ReadRegisterCommand<'b> {
    type Error = error::Error;

    fn try_from(value: frame::Frame<'b>) -> Result<Self> {
        if value.raw_bytes().len() != 8 {
            return Err(Self::Error::InvalidLength);
        }
        if value.function() != function::READ_HOLDING_REGISTERS {
            return Err(Self::Error::WrongFunction);
        }
        Ok(ReadRegisterCommand { frame: value })
    }
}
