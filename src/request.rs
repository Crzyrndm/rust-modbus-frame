use core::ops::Rem;

use crate::{
    builder, function, response, Error, Exception, FixedLen, Frame, Function, FunctionCode,
    PacketLen,
};

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
        start_index: u16,
        count: u16,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .registers([start_index, count])
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

    pub fn coil_count(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[2..])
    }

    pub fn response_builder<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        coils: impl IntoIterator<Item = bool>,
    ) -> (response::ReadCoils<'buff>, &'buff mut [u8]) {
        let (frame, rem) = self
            .frame
            .response_builder(response_buffer)
            .count_bits(coils)
            .finalise();
        (response::ReadCoils::from_frame_unchecked(frame), rem)
    }

    pub fn response_exception<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        exception: Exception,
    ) -> (Frame<'buff>, &'buff mut [u8]) {
        self.frame.response_exception(response_buffer, exception)
    }
}

impl FixedLen for ReadCoils<'_> {
    const LEN: u8 = 8;
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
        start_index: u16,
        count: u16,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .registers([start_index, count])
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

    pub fn input_count(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[2..])
    }

    pub fn response_builder<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        inputs: impl IntoIterator<Item = bool>,
    ) -> (response::ReadDiscreteInputs<'buff>, &'buff mut [u8]) {
        let (frame, rem) = self
            .frame
            .response_builder(response_buffer)
            .count_following_bytes(|mut builder| {
                // LSB is the addressed input with following addresses in order
                let coil_iter = inputs.into_iter().enumerate();

                let mut write_last = false;
                let mut b = 0;
                for (idx, val) in coil_iter {
                    write_last = true;
                    if val {
                        b |= 1 << idx;
                    }
                    if 0 == idx.rem(u8::BITS as usize) {
                        builder = builder.byte(b);
                        b = 0;
                        write_last = false;
                    }
                }
                if write_last {
                    builder = builder.byte(b);
                }

                builder
            })
            .finalise();
        (
            response::ReadDiscreteInputs::from_frame_unchecked(frame),
            rem,
        )
    }

    pub fn response_exception<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        exception: Exception,
    ) -> (Frame<'buff>, &'buff mut [u8]) {
        self.frame.response_exception(response_buffer, exception)
    }
}

impl FixedLen for ReadDiscreteInputs<'_> {
    const LEN: u8 = 8;
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
        start_index: u16,
        register_count: u16,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .registers([start_index, register_count])
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

    pub fn response_builder<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        registers: impl IntoIterator<Item = u16>,
    ) -> (response::ReadHoldingRegisters<'buff>, &'buff mut [u8]) {
        let (frame, rem) = self
            .frame
            .response_builder(response_buffer)
            .count_following_bytes(|builder| builder.registers(registers))
            .finalise();
        (
            response::ReadHoldingRegisters::from_frame_unchecked(frame),
            rem,
        )
    }

    pub fn response_exception<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        exception: Exception,
    ) -> (Frame<'buff>, &'buff mut [u8]) {
        self.frame.response_exception(response_buffer, exception)
    }
}

impl FixedLen for ReadHoldingRegisters<'_> {
    const LEN: u8 = 8;
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
        start_index: u16,
        register_count: u16,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .registers([start_index, register_count])
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

    pub fn response_builder<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        registers: impl IntoIterator<Item = u16>,
    ) -> (response::ReadInputRegisters<'buff>, &'buff mut [u8]) {
        let (frame, rem) = self
            .frame
            .response_builder(response_buffer)
            .count_following_bytes(|builder| builder.registers(registers))
            .finalise();
        (
            response::ReadInputRegisters::from_frame_unchecked(frame),
            rem,
        )
    }

    pub fn response_exception<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        exception: Exception,
    ) -> (Frame<'buff>, &'buff mut [u8]) {
        self.frame.response_exception(response_buffer, exception)
    }
}

impl FixedLen for ReadInputRegisters<'_> {
    const LEN: u8 = 8;
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
        index: u16,
        value: u16,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .registers([index, value])
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

    pub fn is_on(&self) -> bool {
        self.value() == super::COIL_ON
    }

    pub fn response_builder<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
    ) -> (response::WriteCoil<'buff>, &'buff mut [u8]) {
        let (frame, rem) = self
            .frame
            .response_builder(response_buffer)
            .registers([self.index(), self.value()])
            .finalise();
        (response::WriteCoil::from_frame_unchecked(frame), rem)
    }

    pub fn response_exception<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        exception: Exception,
    ) -> (Frame<'buff>, &'buff mut [u8]) {
        self.frame.response_exception(response_buffer, exception)
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
        index: u16,
        value: u16,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .registers([index, value])
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

    pub fn response_builder<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
    ) -> (response::WriteHoldingRegister<'buff>, &'buff mut [u8]) {
        let (frame, rem) = self
            .frame
            .response_builder(response_buffer)
            .registers([self.index(), self.value()])
            .finalise();
        (
            response::WriteHoldingRegister::from_frame_unchecked(frame),
            rem,
        )
    }

    pub fn response_exception<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        exception: Exception,
    ) -> (Frame<'buff>, &'buff mut [u8]) {
        self.frame.response_exception(response_buffer, exception)
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
        start_index: u16,
        coils: impl IntoIterator<Item = bool>,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .register(start_index)
            .count_bits(coils)
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

    pub fn coil_count(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[2..])
    }

    pub fn payload_len(&self) -> u8 {
        self.frame.payload()[4]
    }

    pub fn iter_coils(&'_ self) -> impl Iterator<Item = (u16, bool)> + '_ {
        let data = {
            // location(2) + count(2) + payload_count(1)
            &self.frame.payload()[5..]
        };

        // iteration order is least significant first
        bitvec::slice::BitSlice::<u8, Lsb0>::from_slice(data)
            .iter()
            .enumerate()
            .map(|(idx, bit)| (self.start_index() + idx as u16, *bit))
            .take(self.coil_count().into())
    }

    pub fn response_builder<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
    ) -> (response::WriteMultipleCoils<'buff>, &'buff mut [u8]) {
        let (frame, rem) = self
            .frame
            .response_builder(response_buffer)
            .registers([self.start_index(), self.coil_count()])
            .finalise();
        (
            response::WriteMultipleCoils::from_frame_unchecked(frame),
            rem,
        )
    }

    pub fn response_exception<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        exception: Exception,
    ) -> (Frame<'buff>, &'buff mut [u8]) {
        self.frame.response_exception(response_buffer, exception)
    }
}

impl PacketLen for WriteMultipleCoils<'_> {
    fn packet_len(&self) -> u8 {
        4 + 4 + 1 + self.payload_len()
    }

    // Modbus RTU + payload len + start location + register value
    fn minimum_len() -> u8 {
        9
    }
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
        start_index: u16,
        registers: impl IntoIterator<Item = u16>,
    ) -> (Self, &'a mut [u8]) {
        let (frame, rem) = builder::build_frame(frame_buffer)
            .for_address(address)
            .function(Self::FUNCTION)
            .register(start_index)
            .count_registers(registers)
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
        self.frame.payload()[4]
    }

    pub fn start_index(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[0..])
    }

    pub fn register_count(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.frame.payload()[2..])
    }

    pub fn iter_registers(&'_ self) -> impl Iterator<Item = u16> + '_ {
        self.frame.payload()[5..]
            .chunks(2)
            .map(byteorder::BigEndian::read_u16)
    }

    pub fn response_builder<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
    ) -> (
        response::WriteMultipleHoldingRegisters<'buff>,
        &'buff mut [u8],
    ) {
        let (frame, rem) = self
            .frame
            .response_builder(response_buffer)
            .registers([self.start_index(), self.register_count()])
            .finalise();
        (
            response::WriteMultipleHoldingRegisters::from_frame_unchecked(frame),
            rem,
        )
    }

    pub fn response_exception<'buff>(
        &self,
        response_buffer: &'buff mut [u8],
        exception: Exception,
    ) -> (Frame<'buff>, &'buff mut [u8]) {
        self.frame.response_exception(response_buffer, exception)
    }
}

impl PacketLen for WriteMultipleHoldingRegisters<'_> {
    fn packet_len(&self) -> u8 {
        4 + 4 + 1 + self.payload_len()
    }

    // Modbus RTU + payload len + start location + register value
    fn minimum_len() -> u8 {
        9
    }
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
    use crate::{function, request, COIL_ON};

    #[test]
    fn command_read_coils() {
        let mut buf = [0; 256];
        // 11 01 0013 0025 0E84
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::READ_COILS)
            .registers([0x13, 0x25])
            .finalise();

        let commands = [
            request::ReadCoils::try_from(frame.raw_bytes()).unwrap(),
            request::ReadCoils::try_from(frame).unwrap(),
        ];

        for command in commands {
            assert_eq!(command.start_index(), 0x13);
            assert_eq!(command.coil_count(), 0x25);
        }
    }

    #[test]
    fn command_read_discrete_inputs() {
        let mut buf = [0; 256];
        // 11 02 00C4 0016 BAA9
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::READ_DISCRETE_INPUTS)
            .registers([0xC4, 0x16])
            .finalise();

        let commands = [
            request::ReadDiscreteInputs::try_from(frame.raw_bytes()).unwrap(),
            request::ReadDiscreteInputs::try_from(frame).unwrap(),
        ];

        for command in commands {
            assert_eq!(command.start_index(), 0xC4);
            assert_eq!(command.input_count(), 0x16);
        }
    }

    #[test]
    fn command_read_holding_registers() {
        let mut buf = [0; 256];
        // 11 03 006B 0003 7687
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::READ_HOLDING_REGISTERS)
            .registers([0x6B, 3])
            .finalise();

        let commands = [
            request::ReadHoldingRegisters::try_from(frame.raw_bytes()).unwrap(),
            request::ReadHoldingRegisters::try_from(frame).unwrap(),
        ];

        for command in commands {
            assert_eq!(command.start_index(), 0x6B);
            assert_eq!(command.register_count(), 3);
        }
    }

    #[test]
    fn command_read_input_registers() {
        let mut buf = [0; 256];
        // 11 04 0008 0001 B298
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::READ_INPUT_REGISTERS)
            .registers([8, 1])
            .finalise();

        let commands = [
            request::ReadInputRegisters::try_from(frame.raw_bytes()).unwrap(),
            request::ReadInputRegisters::try_from(frame).unwrap(),
        ];

        for command in commands {
            assert_eq!(command.start_index(), 8);
            assert_eq!(command.register_count(), 1);
        }
    }

    #[test]
    fn command_write_coil() {
        let mut buf = [0; 256];
        // 11 05 00AC FF00 4E8B
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::WRITE_COIL)
            .registers([0xAC, COIL_ON])
            .finalise();

        let commands = [
            request::WriteCoil::try_from(frame.raw_bytes()).unwrap(),
            request::WriteCoil::try_from(frame).unwrap(),
        ];

        for command in commands {
            assert_eq!(command.index(), 0xAC);
            assert!(command.is_on());
        }
    }

    #[test]
    fn command_write_holding_register() {
        let mut buf = [0; 256];
        // 11 06 0001 0003 9A9B
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::WRITE_HOLDING_REGISTER)
            .registers([1, 3])
            .finalise();

        let commands = [
            request::WriteHoldingRegister::try_from(frame.raw_bytes()).unwrap(),
            request::WriteHoldingRegister::try_from(frame).unwrap(),
        ];

        for command in commands {
            assert_eq!(command.index(), 1);
            assert_eq!(command.value(), 3);
        }
    }

    #[test]
    fn command_write_multiple_coils() {
        let mut buf = [0; 256];
        // 0xB, 0xF, 0x0, 0x1B, 0x0, 0x9, 0x2, 0x4D, 0x1, 0x6C, 0xA7
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0xB)
            .function(function::WRITE_MULTIPLE_COILS)
            .registers([27, 9])
            .byte(2)
            .bytes([0x4D, 0x01])
            .finalise();

        let commands = [
            request::WriteMultipleCoils::try_from(frame.raw_bytes()).unwrap(),
            request::WriteMultipleCoils::try_from(frame).unwrap(),
        ];

        for command in commands {
            assert_eq!(command.start_index(), 27);
            assert_eq!(command.coil_count(), 9);
            assert_eq!(command.payload_len(), 2);
            let coils = command.iter_coils().collect::<Vec<_>>();
            let desired = [
                true, false, true, true, false, false, true, false, // 0x4D
                true,  // 0x01
            ]
            .into_iter()
            .enumerate()
            .map(|x| (command.start_index() + x.0 as u16, x.1))
            .collect::<Vec<_>>();
            assert_eq!(coils, desired);
        }
    }

    #[test]
    fn command_write_multiple_holding_registers() {
        let mut buf = [0; 256];
        // 0x11, 0x10, 0x0, 0x01, 0x00, 0x02, 0x04, 0x00, 0x0A, 0x01, 0x02, 0xC6, 0xF0
        let (frame, _remainder) = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::WRITE_MULTIPLE_HOLDING_REGISTERS)
            .registers([1, 2])
            .byte(4)
            .registers([0xA, 0x0102])
            .finalise();

        let commands = [
            request::WriteMultipleHoldingRegisters::try_from(frame.raw_bytes()).unwrap(),
            request::WriteMultipleHoldingRegisters::try_from(frame).unwrap(),
        ];

        for command in commands {
            assert_eq!(command.start_index(), 1);
            assert_eq!(command.register_count(), 2);
            assert_eq!(command.payload_len(), 4);
            let registers = command.iter_registers().collect::<Vec<_>>();
            let desired = [0xA, 0x0102];
            assert_eq!(registers, desired);
        }
    }
}
