use crate::{
    device::Device,
    error,
    rtu::frame::{build_frame, Frame},
    standard::function,
    Result,
};
use core::convert::TryFrom;

#[derive(PartialEq, Debug, Clone)]
pub struct ReadRegisterCommand<'b> {
    frame: Frame<'b>,
}

impl<'b> ReadRegisterCommand<'b> {
    pub fn new(
        buffer: &'b mut [u8],
        device: &Device,
        start_reg: u16,
        num_regs: u16,
    ) -> ReadRegisterCommand<'b> {
        assert!(buffer.len() >= 8); // need at least 8 bytes to format the command into

        let frame = build_frame(buffer)
            .for_device(device)
            .function(function::READ_HOLDING_REGISTERS)
            .register(start_reg)
            .register(num_regs)
            .to_frame();
        ReadRegisterCommand { frame }
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
        Frame::try_from(value).map(|frame| ReadRegisterCommand { frame })
    }
}

impl<'b> TryFrom<Frame<'b>> for ReadRegisterCommand<'b> {
    type Error = error::Error;

    fn try_from(value: Frame<'b>) -> Result<Self> {
        if value.raw_bytes().len() != 8 {
            return Err(Self::Error::InvalidLength);
        }
        if value.function() != function::READ_HOLDING_REGISTERS {
            return Err(Self::Error::WrongFunction);
        }
        Ok(ReadRegisterCommand { frame: value })
    }
}