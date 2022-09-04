use crate::{builder, function, Error, FixedLen, Frame, Function, FunctionCode, PacketLen};

use bitvec::prelude::*;
use byteorder::ByteOrder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadCoils<'a> {
    frame: Frame<'a>,
}

impl<'a> ReadCoils<'a> {
    pub fn new(
        frame_buffer: &'a mut [u8],
        address: u8,
        coils: impl IntoIterator<Item = bool>,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .count_following_bytes(|builder| builder.bits(coils).0)
            .finalise();
        (Self::from_frame_unchecked(frame), rem)
    }

    pub fn from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        Self {
            frame: Frame::new_unchecked(bytes),
        }
    }

    pub fn from_frame_unchecked(frame: Frame<'a>) -> Self {
        Self { frame }
    }

    pub fn as_frame(&self) -> Frame<'a> {
        self.frame
    }

    pub fn payload_len(&self) -> u8 {
        self.frame.payload()[0]
    }

    pub fn iter_coils(&'_ self) -> impl Iterator<Item = bool> + '_ {
        let data = {
            // header(2) + location(2) + count(2) + payload_count(1)
            &self.frame.payload()[1..]
        };
        // the byte ordering for the response here is odd in that it is the Least Significant Bits that are the leftmost
        // this makes the mex appear to zigzag e.g. [CD, 6B, B2, 7F] has the following bit offsets [(7-0), (15-8), (23-16), (30-24)]
        bitvec::slice::BitSlice::<u8, Lsb0>::from_slice(data)
            .iter()
            .map(|bit| *bit)
    }
}

impl PacketLen for ReadCoils<'_> {
    fn packet_len(&self) -> u8 {
        4 + 1 + self.payload_len()
    }

    fn minimum_len() -> u8 {
        5
    }
}

impl FunctionCode for ReadCoils<'_> {
    const FUNCTION: Function = function::READ_COILS;
}

impl<'a> TryFrom<&'a [u8]> for ReadCoils<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::try_from(bytes)?;
        Self::try_from(frame)
    }
}

impl<'a> TryFrom<Frame<'a>> for ReadCoils<'a> {
    type Error = crate::Error;

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        if frame.function() != Self::FUNCTION {
            Err(Error::UnexpectedFunction)
        } else if !Self::is_valid_len(frame.raw_bytes().len()) {
            Err(Error::DecodeInvalidLength)
        } else {
            Ok(Self::from_bytes_unchecked(frame.into_raw_bytes()))
        }
    }
}

impl<'a> From<ReadCoils<'a>> for Frame<'a> {
    fn from(command: ReadCoils<'_>) -> Frame<'_> {
        command.frame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadDiscreteInputs<'a> {
    frame: Frame<'a>,
}

impl<'a> ReadDiscreteInputs<'a> {
    pub fn new(
        frame_buffer: &'a mut [u8],
        address: u8,
        inputs: impl IntoIterator<Item = bool>,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .count_following_bytes(|builder| builder.bits(inputs).0)
            .finalise();
        (Self::from_frame_unchecked(frame), rem)
    }

    pub fn from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        Self {
            frame: Frame::new_unchecked(bytes),
        }
    }

    pub fn from_frame_unchecked(frame: Frame<'a>) -> Self {
        Self { frame }
    }

    pub fn as_frame(&self) -> Frame<'a> {
        self.frame
    }

    pub fn payload_len(&self) -> u8 {
        self.frame.payload()[0]
    }

    pub fn iter_inputs(&'_ self) -> impl Iterator<Item = bool> + '_ {
        let data = {
            // header(2) + location(2) + count(2) + payload_count(1)
            &self.frame.payload()[1..]
        };
        // the byte ordering for the response here is odd in that it is the Least Significant Bits that are the leftmost
        // this makes the mex appear to zigzag e.g. [CD, 6B, B2, 7F] has the following bit offsets [(7-0), (15-8), (23-16), (30-24)]
        bitvec::slice::BitSlice::<u8, Lsb0>::from_slice(data)
            .iter()
            .map(|bit| *bit)
    }
}

impl PacketLen for ReadDiscreteInputs<'_> {
    fn packet_len(&self) -> u8 {
        4 + 1 + self.payload_len()
    }

    fn minimum_len() -> u8 {
        5
    }
}

impl FunctionCode for ReadDiscreteInputs<'_> {
    const FUNCTION: Function = function::READ_DISCRETE_INPUTS;
}

impl<'a> TryFrom<&'a [u8]> for ReadDiscreteInputs<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::try_from(bytes)?;
        Self::try_from(frame)
    }
}

impl<'a> TryFrom<Frame<'a>> for ReadDiscreteInputs<'a> {
    type Error = crate::Error;

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        if frame.function() != Self::FUNCTION {
            Err(Error::UnexpectedFunction)
        } else if !Self::is_valid_len(frame.raw_bytes().len()) {
            Err(Error::DecodeInvalidLength)
        } else {
            Ok(Self::from_bytes_unchecked(frame.into_raw_bytes()))
        }
    }
}

impl<'a> From<ReadDiscreteInputs<'a>> for Frame<'a> {
    fn from(command: ReadDiscreteInputs<'_>) -> Frame<'_> {
        command.frame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadHoldingRegisters<'a> {
    frame: Frame<'a>,
}

impl<'a> ReadHoldingRegisters<'a> {
    pub fn new(
        frame_buffer: &'a mut [u8],
        address: u8,
        registers: impl IntoIterator<Item = u16>,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .count_following_bytes(|builder| builder.registers(registers))
            .finalise();
        (Self::from_frame_unchecked(frame), rem)
    }

    pub fn from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        Self {
            frame: Frame::new_unchecked(bytes),
        }
    }

    pub fn from_frame_unchecked(frame: Frame<'a>) -> Self {
        Self { frame }
    }

    pub fn as_frame(&self) -> Frame<'a> {
        self.frame
    }

    pub fn payload_len(&self) -> u8 {
        self.frame.payload()[0]
    }

    pub fn iter_registers(&'_ self) -> impl Iterator<Item = u16> + '_ {
        self.frame.payload()[1..]
            .chunks(2)
            .map(byteorder::BigEndian::read_u16)
    }
}

impl PacketLen for ReadHoldingRegisters<'_> {
    fn packet_len(&self) -> u8 {
        4 + 1 + self.payload_len()
    }

    fn minimum_len() -> u8 {
        5
    }
}

impl FunctionCode for ReadHoldingRegisters<'_> {
    const FUNCTION: Function = function::READ_HOLDING_REGISTERS;
}

impl<'a> TryFrom<&'a [u8]> for ReadHoldingRegisters<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::try_from(bytes)?;
        Self::try_from(frame)
    }
}

impl<'a> TryFrom<Frame<'a>> for ReadHoldingRegisters<'a> {
    type Error = crate::Error;

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        if frame.function() != Self::FUNCTION {
            Err(Error::UnexpectedFunction)
        } else if !Self::is_valid_len(frame.raw_bytes().len()) {
            Err(Error::DecodeInvalidLength)
        } else {
            Ok(Self::from_bytes_unchecked(frame.into_raw_bytes()))
        }
    }
}

impl<'a> From<ReadHoldingRegisters<'a>> for Frame<'a> {
    fn from(command: ReadHoldingRegisters<'_>) -> Frame<'_> {
        command.frame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadInputRegisters<'a> {
    frame: Frame<'a>,
}

impl<'a> ReadInputRegisters<'a> {
    pub fn new(
        frame_buffer: &'a mut [u8],
        address: u8,
        registers: impl IntoIterator<Item = u16>,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .count_following_bytes(|builder| builder.registers(registers))
            .finalise();
        (Self::from_frame_unchecked(frame), rem)
    }

    pub fn from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        Self {
            frame: Frame::new_unchecked(bytes),
        }
    }

    pub fn from_frame_unchecked(frame: Frame<'a>) -> Self {
        Self { frame }
    }

    pub fn as_frame(&self) -> Frame<'a> {
        self.frame
    }

    pub fn payload_len(&self) -> u8 {
        self.frame.payload()[0]
    }

    pub fn iter_registers(&'_ self) -> impl Iterator<Item = u16> + '_ {
        self.frame.payload()[1..]
            .chunks(2)
            .map(byteorder::BigEndian::read_u16)
    }
}

impl PacketLen for ReadInputRegisters<'_> {
    fn packet_len(&self) -> u8 {
        4 + 1 + self.payload_len()
    }

    fn minimum_len() -> u8 {
        5
    }
}

impl FunctionCode for ReadInputRegisters<'_> {
    const FUNCTION: Function = function::READ_INPUT_REGISTERS;
}

impl<'a> TryFrom<&'a [u8]> for ReadInputRegisters<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::try_from(bytes)?;
        Self::try_from(frame)
    }
}

impl<'a> TryFrom<Frame<'a>> for ReadInputRegisters<'a> {
    type Error = crate::Error;

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        if frame.function() != Self::FUNCTION {
            Err(Error::UnexpectedFunction)
        } else if !Self::is_valid_len(frame.raw_bytes().len()) {
            Err(Error::DecodeInvalidLength)
        } else {
            Ok(Self::from_bytes_unchecked(frame.into_raw_bytes()))
        }
    }
}

impl<'a> From<ReadInputRegisters<'a>> for Frame<'a> {
    fn from(command: ReadInputRegisters<'_>) -> Frame<'_> {
        command.frame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WriteCoil<'a> {
    frame: Frame<'a>,
}

impl<'a> WriteCoil<'a> {
    pub fn new(
        frame_buffer: &'a mut [u8],
        address: u8,
        coil_address: u16,
        write_on: bool,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .registers([
                coil_address,
                if write_on {
                    crate::COIL_ON
                } else {
                    crate::COIL_OFF
                },
            ])
            .finalise();
        (Self::from_frame_unchecked(frame), rem)
    }

    pub fn from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        Self {
            frame: Frame::new_unchecked(bytes),
        }
    }

    pub fn from_frame_unchecked(frame: Frame<'a>) -> Self {
        Self { frame }
    }

    pub fn as_frame(&self) -> Frame<'a> {
        self.frame
    }

    pub fn index(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[0..])
    }

    pub fn is_on(&self) -> bool {
        byteorder::BigEndian::read_u16(&self.frame.payload()[2..]) == super::COIL_ON
    }
}

impl FixedLen for WriteCoil<'_> {
    // Modbus RTU + start location + coil true/false
    const LEN: u8 = 8;
}

impl FunctionCode for WriteCoil<'_> {
    const FUNCTION: Function = function::WRITE_COIL;
}

impl<'a> TryFrom<&'a [u8]> for WriteCoil<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::try_from(bytes)?;
        Self::try_from(frame)
    }
}

impl<'a> TryFrom<Frame<'a>> for WriteCoil<'a> {
    type Error = crate::Error;

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        if frame.function() != Self::FUNCTION {
            Err(Error::UnexpectedFunction)
        } else if !Self::is_valid_len(frame.raw_bytes().len()) {
            Err(Error::DecodeInvalidLength)
        } else {
            Ok(Self::from_bytes_unchecked(frame.into_raw_bytes()))
        }
    }
}

impl<'a> From<WriteCoil<'a>> for Frame<'a> {
    fn from(command: WriteCoil<'_>) -> Frame<'_> {
        command.frame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WriteHoldingRegister<'a> {
    frame: Frame<'a>,
}

impl<'a> WriteHoldingRegister<'a> {
    pub fn new(
        frame_buffer: &'a mut [u8],
        address: u8,
        register_address: u16,
        register_value: u16,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .registers([register_address, register_value])
            .finalise();
        (Self::from_frame_unchecked(frame), rem)
    }

    pub fn from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        Self {
            frame: Frame::new_unchecked(bytes),
        }
    }

    pub fn from_frame_unchecked(frame: Frame<'a>) -> Self {
        Self { frame }
    }

    pub fn as_frame(&self) -> Frame<'a> {
        self.frame
    }

    pub fn index(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[0..])
    }

    pub fn value(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[2..])
    }
}

impl FixedLen for WriteHoldingRegister<'_> {
    // Modbus RTU + start location + register value
    const LEN: u8 = 8;
}

impl FunctionCode for WriteHoldingRegister<'_> {
    const FUNCTION: Function = function::WRITE_HOLDING_REGISTER;
}

impl<'a> TryFrom<&'a [u8]> for WriteHoldingRegister<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::try_from(bytes)?;
        Self::try_from(frame)
    }
}

impl<'a> TryFrom<Frame<'a>> for WriteHoldingRegister<'a> {
    type Error = crate::Error;

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        if frame.function() != Self::FUNCTION {
            Err(Error::UnexpectedFunction)
        } else if !Self::is_valid_len(frame.raw_bytes().len()) {
            Err(Error::DecodeInvalidLength)
        } else {
            Ok(Self::from_bytes_unchecked(frame.into_raw_bytes()))
        }
    }
}

impl<'a> From<WriteHoldingRegister<'a>> for Frame<'a> {
    fn from(command: WriteHoldingRegister<'_>) -> Frame<'_> {
        command.frame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WriteMultipleCoils<'a> {
    frame: Frame<'a>,
}

impl<'a> WriteMultipleCoils<'a> {
    pub fn new(
        frame_buffer: &'a mut [u8],
        address: u8,
        start_address: u16,
        coil_count: u16,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .registers([start_address, coil_count])
            .finalise();
        (Self::from_frame_unchecked(frame), rem)
    }

    pub fn from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        Self {
            frame: Frame::new_unchecked(bytes),
        }
    }

    pub fn from_frame_unchecked(frame: Frame<'a>) -> Self {
        Self { frame }
    }

    pub fn as_frame(&self) -> Frame<'a> {
        self.frame
    }

    pub fn start_index(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[0..])
    }

    pub fn register_count(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[2..])
    }
}

impl FixedLen for WriteMultipleCoils<'_> {
    const LEN: u8 = 8;
}

impl FunctionCode for WriteMultipleCoils<'_> {
    const FUNCTION: Function = function::WRITE_MULTIPLE_COILS;
}

impl<'a> TryFrom<&'a [u8]> for WriteMultipleCoils<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::try_from(bytes)?;
        Self::try_from(frame)
    }
}

impl<'a> TryFrom<Frame<'a>> for WriteMultipleCoils<'a> {
    type Error = crate::Error;

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        if frame.function() != Self::FUNCTION {
            Err(Error::UnexpectedFunction)
        } else if !Self::is_valid_len(frame.raw_bytes().len()) {
            Err(Error::DecodeInvalidLength)
        } else {
            Ok(Self::from_bytes_unchecked(frame.into_raw_bytes()))
        }
    }
}

impl<'a> From<WriteMultipleCoils<'a>> for Frame<'a> {
    fn from(command: WriteMultipleCoils<'_>) -> Frame<'_> {
        command.frame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WriteMultipleHoldingRegisters<'a> {
    frame: Frame<'a>,
}

impl<'a> WriteMultipleHoldingRegisters<'a> {
    pub fn new(
        frame_buffer: &'a mut [u8],
        address: u8,
        start_address: u16,
        register_count: u16,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .registers([start_address, register_count])
            .finalise();
        (Self::from_frame_unchecked(frame), rem)
    }

    pub fn from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        Self {
            frame: Frame::new_unchecked(bytes),
        }
    }

    pub fn from_frame_unchecked(frame: Frame<'a>) -> Self {
        Self { frame }
    }

    pub fn as_frame(&self) -> Frame<'a> {
        self.frame
    }

    pub fn start_index(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[0..])
    }

    pub fn register_count(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[2..])
    }
}

impl FixedLen for WriteMultipleHoldingRegisters<'_> {
    const LEN: u8 = 8;
}

impl FunctionCode for WriteMultipleHoldingRegisters<'_> {
    const FUNCTION: Function = function::WRITE_MULTIPLE_HOLDING_REGISTERS;
}

impl<'a> TryFrom<&'a [u8]> for WriteMultipleHoldingRegisters<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let frame = Frame::try_from(bytes)?;
        Self::try_from(frame)
    }
}

impl<'a> TryFrom<Frame<'a>> for WriteMultipleHoldingRegisters<'a> {
    type Error = crate::Error;

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        if frame.function() != Self::FUNCTION {
            Err(Error::UnexpectedFunction)
        } else if !Self::is_valid_len(frame.raw_bytes().len()) {
            Err(Error::DecodeInvalidLength)
        } else {
            Ok(Self::from_bytes_unchecked(frame.into_raw_bytes()))
        }
    }
}

impl<'a> From<WriteMultipleHoldingRegisters<'a>> for Frame<'a> {
    fn from(command: WriteMultipleHoldingRegisters<'_>) -> Frame<'_> {
        command.frame
    }
}

#[cfg(test)]
mod tests {
    use crate::{function, response, COIL_ON};

    #[test]
    fn response_read_coils() {
        let mut buf = [0; 256];
        // 0xB, 0x1, 0x4, 0xCD, 0x6B, 0xB2, 0x7F, 0x2B, 0xE1
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0xB)
            .function(function::READ_COILS)
            .byte(4)
            .bytes([0xCD, 0x6B, 0xB2, 0x7F])
            .finalise();
        let responses = [
            response::ReadCoils::try_from(frame.raw_bytes()).unwrap(),
            response::ReadCoils::try_from(frame).unwrap(),
        ];

        for response in responses {
            assert_eq!(response.payload_len(), 4);
            let coils = response.iter_coils().collect::<Vec<_>>();
            let desired = [
                true, false, true, true, false, false, true, true, // 0xCD
                true, true, false, true, false, true, true, false, // 0x6B
                false, true, false, false, true, true, false, true, // 0xB2
                true, true, true, true, true, true, true, false, // 0x7F
            ];
            // Note that the last false bit may be padding (always zeroes) or part of the message. Need to know what the request was to tell
            assert_eq!(coils, desired);
        }
    }

    #[test]
    fn response_read_discrete_inputs() {
        let mut buf = [0; 256];

        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0xB)
            .function(function::READ_DISCRETE_INPUTS)
            .byte(4)
            .bytes([0xCD, 0x6B, 0xB2, 0x7F])
            .finalise();
        let responses = [
            response::ReadDiscreteInputs::try_from(frame.raw_bytes()).unwrap(),
            response::ReadDiscreteInputs::try_from(frame).unwrap(),
        ];

        for response in responses {
            assert_eq!(response.payload_len(), 4);
            let coils = response.iter_inputs().collect::<Vec<_>>();
            let desired = [
                true, false, true, true, false, false, true, true, // 0xCD
                true, true, false, true, false, true, true, false, // 0x6B
                false, true, false, false, true, true, false, true, // 0xB2
                true, true, true, true, true, true, true, false, // 0x7F
            ];
            // Note that the last false bit may be padding (always zeroes) or part of the message. Need to know what the request was to tell
            assert_eq!(coils, desired);
        }
    }

    #[test]
    fn response_read_holding_registers() {
        let mut buf = [0; 256];
        // 11 03 06 AE41 5652 4340 49AD
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::READ_HOLDING_REGISTERS)
            .byte(6)
            .registers([0xAE41, 0x5652, 0x4340])
            .finalise();

        let responses = [
            response::ReadHoldingRegisters::try_from(frame.raw_bytes()).unwrap(),
            response::ReadHoldingRegisters::try_from(frame).unwrap(),
        ];

        for response in responses {
            assert_eq!(response.payload_len(), 6);
            assert_eq!(
                response.iter_registers().collect::<Vec<_>>(),
                [0xAE41, 0x5652, 0x4340]
            );
        }
    }

    #[test]
    fn response_read_input_registers() {
        let mut buf = [0; 256];
        // 11 03 06 AE41 5652 4340 49AD
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::READ_INPUT_REGISTERS)
            .byte(6)
            .registers([0xAE41, 0x5652, 0x4340])
            .finalise();

        let responses = [
            response::ReadInputRegisters::try_from(frame.raw_bytes()).unwrap(),
            response::ReadInputRegisters::try_from(frame).unwrap(),
        ];

        for response in responses {
            assert_eq!(response.payload_len(), 6);
            assert_eq!(
                response.iter_registers().collect::<Vec<_>>(),
                [0xAE41, 0x5652, 0x4340]
            );
        }
    }

    #[test]
    fn response_write_coil() {
        let mut buf = [0; 256];
        // 11 05 00AC FF00 4E8B
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::WRITE_COIL)
            .registers([0xAC, COIL_ON])
            .finalise();

        let responses = [
            response::WriteCoil::try_from(frame.raw_bytes()).unwrap(),
            response::WriteCoil::try_from(frame).unwrap(),
        ];

        for response in responses {
            assert_eq!(response.index(), 0xAC);
            assert!(response.is_on());
        }
    }

    #[test]
    fn response_write_holding_register() {
        let mut buf = [0; 256];
        // 11 06 0001 0003 9A9B
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::WRITE_HOLDING_REGISTER)
            .registers([1, 3])
            .finalise();

        let responses = [
            response::WriteHoldingRegister::try_from(frame.raw_bytes()).unwrap(),
            response::WriteHoldingRegister::try_from(frame).unwrap(),
        ];

        for response in responses {
            assert_eq!(response.index(), 1);
            assert_eq!(response.value(), 3);
        }
    }

    #[test]
    fn response_write_multiple_coils() {
        let mut buf = [0; 256];
        // 0xB, 0xF, 0x0, 0x1B, 0x0, 0x9, 0x2, 0x4D, 0x1, 0x6C, 0xA7
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0xB)
            .function(function::WRITE_MULTIPLE_COILS)
            .registers([27, 9])
            .finalise();

        let responses = [
            response::WriteMultipleCoils::try_from(frame.raw_bytes()).unwrap(),
            response::WriteMultipleCoils::try_from(frame).unwrap(),
        ];

        for response in responses {
            assert_eq!(response.start_index(), 27);
            assert_eq!(response.register_count(), 9);
        }
    }

    #[test]
    fn response_write_multiple_holding_registers() {
        let mut buf = [0; 256];
        // 0x11, 0x10, 0x0, 0x01, 0x00, 0x02, 0x04, 0x00, 0x0A, 0x01, 0x02, 0xC6, 0xF0
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::WRITE_MULTIPLE_HOLDING_REGISTERS)
            .registers([1, 2])
            .finalise();

        let responses = [
            response::WriteMultipleHoldingRegisters::try_from(frame.raw_bytes()).unwrap(),
            response::WriteMultipleHoldingRegisters::try_from(frame).unwrap(),
        ];

        for response in responses {
            assert_eq!(response.start_index(), 1);
            assert_eq!(response.register_count(), 2);
        }
    }
}
