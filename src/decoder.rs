//! Take bytes, turn into outputs

use crate::{frame::Frame, function, request, response, Error};

/// The default responses for a decode type
/// ```
/// use modbus_frames::{builder, function, decoder::CommonRequests};
/// # let mut buf = [0; 256];
/// let (command_frame, rem) = builder::build_frame(&mut buf)
///                        .for_address(0x11)
///                        .function(function::READ_COILS)
///                        .registers([0x13, 0x25])
///                        .finalise();
/// let decoded = CommonRequests::try_from(command_frame).unwrap();
/// assert!(matches!(decoded, CommonRequests::ReadCoils(_)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommonRequests<'a> {
    ReadCoils(request::ReadCoils<'a>),
    ReadDiscreteInputs(request::ReadDiscreteInputs<'a>),
    ReadHolsingRegisters(request::ReadHoldingRegisters<'a>),
    ReadInputRegisters(request::ReadInputRegisters<'a>),
    WriteCoil(request::WriteCoil<'a>),
    WriteHoldingRegister(request::WriteHoldingRegister<'a>),
    WriteMultipleCoils(request::WriteMultipleCoils<'a>),
    WriteMultipleHoldingRegisters(request::WriteMultipleHoldingRegisters<'a>),
}

impl<'a> CommonRequests<'a> {
    pub fn as_frame(&self) -> Frame {
        (*self).into()
    }
}

impl<'a> From<CommonRequests<'a>> for Frame<'a> {
    fn from(response: CommonRequests<'a>) -> Self {
        match response {
            CommonRequests::ReadCoils(res) => res.as_frame(),
            CommonRequests::ReadDiscreteInputs(res) => res.as_frame(),
            CommonRequests::ReadHolsingRegisters(res) => res.as_frame(),
            CommonRequests::ReadInputRegisters(res) => res.as_frame(),
            CommonRequests::WriteCoil(res) => res.as_frame(),
            CommonRequests::WriteHoldingRegister(res) => res.as_frame(),
            CommonRequests::WriteMultipleCoils(res) => res.as_frame(),
            CommonRequests::WriteMultipleHoldingRegisters(res) => res.as_frame(),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for CommonRequests<'a> {
    // all Ok types are returning at least three pointers (discriminant, slice start, slice len)
    // can put a bit of info in the error type with no change in size
    // e.g. which function code
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::try_from(bytes)?;
        Self::try_from(frame)
    }
}

impl<'a> TryFrom<Frame<'a>> for CommonRequests<'a> {
    type Error = crate::Error;

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        match frame.function() {
            function::READ_COILS => request::ReadCoils::try_from(frame).map(Self::ReadCoils),
            function::READ_DISCRETE_INPUTS => {
                request::ReadDiscreteInputs::try_from(frame).map(Self::ReadDiscreteInputs)
            }
            function::READ_HOLDING_REGISTERS => {
                request::ReadHoldingRegisters::try_from(frame).map(Self::ReadHolsingRegisters)
            }
            function::READ_INPUT_REGISTERS => {
                request::ReadInputRegisters::try_from(frame).map(Self::ReadInputRegisters)
            }
            function::WRITE_COIL => request::WriteCoil::try_from(frame).map(Self::WriteCoil),
            function::WRITE_HOLDING_REGISTER => {
                request::WriteHoldingRegister::try_from(frame).map(Self::WriteHoldingRegister)
            }
            function::WRITE_MULTIPLE_COILS => {
                request::WriteMultipleCoils::try_from(frame).map(Self::WriteMultipleCoils)
            }
            function::WRITE_MULTIPLE_HOLDING_REGISTERS => {
                request::WriteMultipleHoldingRegisters::try_from(frame)
                    .map(Self::WriteMultipleHoldingRegisters)
            }
            // unknwn function code
            _ => Err(Error::UnknownFunction),
        }
    }
}

/// The default responses for a decode type
/// ```
/// use modbus_frames::{builder, function, decoder::CommonResponses};
/// # let mut buf = [0; 256];
/// let response_frame = builder::build_frame(&mut buf)
///            .for_address(0xB)
///            .function(function::READ_COILS)
///            .byte(4)
///            .bytes([0xCD, 0x6B, 0xB2, 0x7F])
///            .finalise();
/// let decoded = CommonResponses::try_from(response_frame.0).unwrap();
/// assert!(matches!(decoded, CommonResponses::ReadCoils(_)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommonResponses<'a> {
    ReadCoils(response::ReadCoils<'a>),
    ReadDiscreteInputs(response::ReadDiscreteInputs<'a>),
    ReadHolsingRegisters(response::ReadHoldingRegisters<'a>),
    ReadInputRegisters(response::ReadInputRegisters<'a>),
    WriteCoil(response::WriteCoil<'a>),
    WriteHoldingRegister(response::WriteHoldingRegister<'a>),
    WriteMultipleCoils(response::WriteMultipleCoils<'a>),
    WriteMultipleHoldingRegisters(response::WriteMultipleHoldingRegisters<'a>),
}

impl<'a> CommonResponses<'a> {
    pub fn as_frame(&self) -> Frame {
        (*self).into()
    }
}

impl<'a> From<CommonResponses<'a>> for Frame<'a> {
    fn from(response: CommonResponses<'a>) -> Self {
        match response {
            CommonResponses::ReadCoils(res) => res.as_frame(),
            CommonResponses::ReadDiscreteInputs(res) => res.as_frame(),
            CommonResponses::ReadHolsingRegisters(res) => res.as_frame(),
            CommonResponses::ReadInputRegisters(res) => res.as_frame(),
            CommonResponses::WriteCoil(res) => res.as_frame(),
            CommonResponses::WriteHoldingRegister(res) => res.as_frame(),
            CommonResponses::WriteMultipleCoils(res) => res.as_frame(),
            CommonResponses::WriteMultipleHoldingRegisters(res) => res.as_frame(),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for CommonResponses<'a> {
    // all Ok types are returning at least three pointers (discriminant, slice start, slice len)
    // can put a bit of info in the error type with no change in size
    // e.g. which function code
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::try_from(bytes)?;
        Self::try_from(frame)
    }
}

impl<'a> TryFrom<Frame<'a>> for CommonResponses<'a> {
    type Error = crate::Error;

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        match frame.function() {
            function::READ_COILS => response::ReadCoils::try_from(frame).map(Self::ReadCoils),
            function::READ_DISCRETE_INPUTS => {
                response::ReadDiscreteInputs::try_from(frame).map(Self::ReadDiscreteInputs)
            }
            function::READ_HOLDING_REGISTERS => {
                response::ReadHoldingRegisters::try_from(frame).map(Self::ReadHolsingRegisters)
            }
            function::READ_INPUT_REGISTERS => {
                response::ReadInputRegisters::try_from(frame).map(Self::ReadInputRegisters)
            }
            function::WRITE_COIL => response::WriteCoil::try_from(frame).map(Self::WriteCoil),
            function::WRITE_HOLDING_REGISTER => {
                response::WriteHoldingRegister::try_from(frame).map(Self::WriteHoldingRegister)
            }
            function::WRITE_MULTIPLE_COILS => {
                response::WriteMultipleCoils::try_from(frame).map(Self::WriteMultipleCoils)
            }
            function::WRITE_MULTIPLE_HOLDING_REGISTERS => {
                response::WriteMultipleHoldingRegisters::try_from(frame)
                    .map(Self::WriteMultipleHoldingRegisters)
            }
            // unknwn function code
            _ => Err(Error::UnknownFunction),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        decoder::{CommonRequests, CommonResponses},
        function, Frame, COIL_ON,
    };

    #[test]
    fn command_decode() {
        let mut buf = [0; 256];
        // for now, just copy the builder sequences from the other tests
        let commands = [
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_COILS)
                .registers([0x13, 0x25])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_DISCRETE_INPUTS)
                .registers([0xC4, 0x16])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_HOLDING_REGISTERS)
                .registers([0x6B, 3])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_INPUT_REGISTERS)
                .registers([8, 1])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_COIL)
                .registers([0xAC, COIL_ON])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_HOLDING_REGISTER)
                .registers([1, 3])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0xB)
                .function(function::WRITE_MULTIPLE_COILS)
                .registers([27, 9])
                .byte(2)
                .bytes([0x4D, 0x01])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_MULTIPLE_HOLDING_REGISTERS)
                .registers([1, 2])
                .byte(4)
                .registers([0xA, 0x0102])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
        ];
        let result = commands
            .into_iter()
            .map(|bytes| {
                let byte = CommonRequests::try_from(bytes.as_slice()).unwrap();
                let frame = Frame::new_unchecked(&bytes);
                let frame = CommonRequests::try_from(frame).unwrap();
                [format!("{:?}", byte), format!("{:?}", frame)]
            })
            .collect::<Vec<_>>();
        dbg!(result);
    }

    #[test]
    fn response_decode() {
        let mut buf = [0; 256];
        // for now, just copy the builder sequences from the other tests
        let responses = [
            crate::builder::build_frame(&mut buf)
                .for_address(0xB)
                .function(function::READ_COILS)
                .byte(4)
                .bytes([0xCD, 0x6B, 0xB2, 0x7F])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0xB)
                .function(function::READ_DISCRETE_INPUTS)
                .byte(4)
                .bytes([0xCD, 0x6B, 0xB2, 0x7F])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_HOLDING_REGISTERS)
                .byte(6)
                .registers([0xAE41, 0x5652, 0x4340])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_INPUT_REGISTERS)
                .byte(6)
                .registers([0xAE41, 0x5652, 0x4340])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_COIL)
                .registers([0xAC, COIL_ON])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_HOLDING_REGISTER)
                .registers([1, 3])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0xB)
                .function(function::WRITE_MULTIPLE_COILS)
                .registers([27, 9])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_MULTIPLE_HOLDING_REGISTERS)
                .registers([1, 2])
                .finalise()
                .0
                .raw_bytes()
                .to_vec(),
        ];
        let result = responses
            .into_iter()
            .map(|bytes| {
                let byte = CommonResponses::try_from(bytes.as_slice()).unwrap();
                let frame = Frame::new_unchecked(&bytes);
                let frame = CommonResponses::try_from(frame).unwrap();
                [format!("{:?}", byte), format!("{:?}", frame)]
            })
            .collect::<Vec<_>>();
        dbg!(result);
    }
}
