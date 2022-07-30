//! Take bytes, turn into outputs

use byteorder::ByteOrder;

use crate::{frame::Frame, function, Error, FixedLen, Function, FunctionCode, PacketLen};

fn try_from_bytes<'a, T>(bytes: &'a [u8]) -> crate::Result<T>
where
    T: 'a + PacketLen + FunctionCode + TryFrom<Frame<'a>, Error = crate::Error>,
{
    let frame = Frame::try_from(bytes)?;
    T::try_from(frame)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WriteCoil<'a> {
    bytes: &'a [u8],
}

impl<'a> WriteCoil<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        debug_assert!(
            // 4 bytes for RTU, 4 bytes of payload
            bytes.len() == 4 + 4,
            "length validation should be done before constructing an instance"
        );
        Self { bytes }
    }

    pub fn start_index(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.bytes[2..])
    }

    pub fn register_count(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.bytes[4..])
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
        try_from_bytes(bytes)
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
            Ok(Self::new(frame.into_raw_bytes()))
        }
    }
}

impl<'a> From<WriteCoil<'a>> for Frame<'a> {
    fn from(command: WriteCoil<'_>) -> Frame<'_> {
        let bytes = command.bytes;
        Frame::new_unchecked(bytes)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WriteHoldingRegister<'a> {
    bytes: &'a [u8],
}

impl<'a> WriteHoldingRegister<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        debug_assert!(
            // 4 bytes for RTU, 4 bytes of payload
            bytes.len() == 4 + 4,
            "length validation should be done before constructing an instance"
        );
        Self { bytes }
    }

    pub fn start_index(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.bytes[2..])
    }

    pub fn register_count(&self) -> u16 {
        byteorder::BigEndian::read_u16(&self.bytes[4..])
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
        try_from_bytes(bytes)
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
            Ok(Self::new(frame.into_raw_bytes()))
        }
    }
}

impl<'a> From<WriteHoldingRegister<'a>> for Frame<'a> {
    fn from(command: WriteHoldingRegister<'_>) -> Frame<'_> {
        let bytes = command.bytes;
        Frame::new_unchecked(bytes)
    }
}

pub mod command {
    use bitvec::macros::internal::funty::Fundamental;
    use bitvec::order::Msb0;
    use byteorder::ByteOrder;

    use crate::frame::Frame;
    use crate::{function, Error, FixedLen, Function, FunctionCode, PacketLen};

    /// The default responses for a decode type
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum CommonCommands<'a> {
        ReadCoils(ReadCoils<'a>),
        ReadDiscreteInputs(ReadDiscreteInputs<'a>),
        ReadHolsingRegisters(ReadHoldingRegisters<'a>),
        ReadInputRegisters(ReadInputRegisters<'a>),
        WriteCoil(WriteCoil<'a>),
        WriteHoldingRegister(WriteHoldingRegister<'a>),
        WriteMultipleCoils(WriteMultipleCoils<'a>),
        WriteMultipleHoldingRegisters(WriteMultipleHoldingRegisters<'a>),
    }

    impl<'a> TryFrom<&'a [u8]> for CommonCommands<'a> {
        // all Ok types are returning at least three pointers (discriminant, slice start, slice len)
        // can put a bit of info in the error type with no change in size
        // e.g. which function code
        type Error = crate::Error;

        fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
            let frame = Frame::try_from(bytes)?;
            Self::try_from(frame)
        }
    }

    impl<'a> TryFrom<Frame<'a>> for CommonCommands<'a> {
        type Error = crate::Error;

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            match frame.function() {
                function::READ_COILS => ReadCoils::try_from(frame).map(Self::ReadCoils),
                function::READ_INPUTS => {
                    ReadDiscreteInputs::try_from(frame).map(Self::ReadDiscreteInputs)
                }
                function::READ_HOLDING_REGISTERS => {
                    ReadHoldingRegisters::try_from(frame).map(Self::ReadHolsingRegisters)
                }
                function::READ_INPUT_REGISTERS => {
                    ReadInputRegisters::try_from(frame).map(Self::ReadInputRegisters)
                }
                function::WRITE_COIL => WriteCoil::try_from(frame).map(Self::WriteCoil),
                function::WRITE_HOLDING_REGISTER => {
                    WriteHoldingRegister::try_from(frame).map(Self::WriteHoldingRegister)
                }
                function::WRITE_MULTIPLE_COILS => {
                    WriteMultipleCoils::try_from(frame).map(Self::WriteMultipleCoils)
                }
                function::WRITE_MULTIPLE_HOLDING_REGISTERS => {
                    WriteMultipleHoldingRegisters::try_from(frame)
                        .map(Self::WriteMultipleHoldingRegisters)
                }
                // unknwn function code
                _ => Err(Error::UnknownFunction),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ReadCoils<'a> {
        bytes: &'a [u8],
    }

    impl<'a> ReadCoils<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() == 4 + 4,
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn start_index(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[2..])
        }

        pub fn coil_count(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[4..])
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
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<ReadCoils<'a>> for Frame<'a> {
        fn from(command: ReadCoils<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ReadDiscreteInputs<'a> {
        bytes: &'a [u8],
    }

    impl<'a> ReadDiscreteInputs<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() == 4 + 4,
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn start_index(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[2..])
        }

        pub fn input_count(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[4..])
        }
    }

    impl FixedLen for ReadDiscreteInputs<'_> {
        const LEN: u8 = 8;
    }

    impl FunctionCode for ReadDiscreteInputs<'_> {
        const FUNCTION: Function = function::READ_INPUTS;
    }

    impl<'a> TryFrom<&'a [u8]> for ReadDiscreteInputs<'a> {
        type Error = crate::Error;

        fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<ReadDiscreteInputs<'a>> for Frame<'a> {
        fn from(command: ReadDiscreteInputs<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ReadHoldingRegisters<'a> {
        bytes: &'a [u8],
    }

    impl<'a> ReadHoldingRegisters<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() == 4 + 4,
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn start_index(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[2..])
        }

        pub fn register_count(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[4..])
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
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<ReadHoldingRegisters<'a>> for Frame<'a> {
        fn from(command: ReadHoldingRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ReadInputRegisters<'a> {
        bytes: &'a [u8],
    }

    impl<'a> ReadInputRegisters<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() == 4 + 4,
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn start_index(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[2..])
        }

        pub fn register_count(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[4..])
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
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<ReadInputRegisters<'a>> for Frame<'a> {
        fn from(command: ReadInputRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }

    use super::WriteHoldingRegister;
    use super::{try_from_bytes, WriteCoil};

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct WriteMultipleCoils<'a> {
        bytes: &'a [u8],
    }

    impl<'a> WriteMultipleCoils<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                bytes.len() >= Self::minimum_len().into(),
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn payload_len(&self) -> u8 {
            self.bytes[6]
        }

        pub fn start_index(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[2..])
        }

        pub fn coil_count(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[4..])
        }

        pub fn iter_coils(&'_ self) -> impl Iterator<Item = (u16, bool)> + '_ {
            let data = {
                // header(2) + location(2) + count(2) + payload_count(1)
                let start_idx = 7;
                let end_idx = self.bytes.len() - 2;
                &self.bytes[start_idx..end_idx]
            };

            // iteration order is from right to left (least significant first)
            bitvec::slice::BitSlice::<u8, Msb0>::from_slice(data)
                .iter()
                .enumerate()
                .map(|(idx, bit)| (self.start_index() + idx as u16, bit.as_bool()))
                .take(self.coil_count().into())
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
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<WriteMultipleCoils<'a>> for Frame<'a> {
        fn from(command: WriteMultipleCoils<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct WriteMultipleHoldingRegisters<'a> {
        bytes: &'a [u8],
    }

    impl<'a> WriteMultipleHoldingRegisters<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() == 4 + 4,
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn payload_len(&self) -> u8 {
            self.bytes[6]
        }

        pub fn start_index(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[2..])
        }

        pub fn register_count(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[4..])
        }

        pub fn iter_registers(&'_ self) -> impl Iterator<Item = u16> + '_ {
            self.bytes[3..]
                .windows(2)
                .map(byteorder::BigEndian::read_u16)
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
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<WriteMultipleHoldingRegisters<'a>> for Frame<'a> {
        fn from(command: WriteMultipleHoldingRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }
}

pub mod response {
    use bitvec::order::{Lsb0, Msb0};
    use byteorder::ByteOrder;

    use super::try_from_bytes;
    pub use super::{WriteCoil, WriteHoldingRegister};

    use crate::{frame::Frame, function, Error, FixedLen, Function, FunctionCode, PacketLen};

    /// The default responses for a decode type
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum CommonResponses<'a> {
        ReadCoils(ReadCoils<'a>),
        ReadDiscreteInputs(ReadDiscreteInputs<'a>),
        ReadHolsingRegisters(ReadHoldingRegisters<'a>),
        ReadInputRegisters(ReadInputRegisters<'a>),
        WriteCoil(WriteCoil<'a>),
        WriteHoldingRegister(WriteHoldingRegister<'a>),
        WriteMultipleCoils(WriteMultipleCoils<'a>),
        WriteMultipleHoldingRegisters(WriteMultipleHoldingRegisters<'a>),
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
                function::READ_COILS => ReadCoils::try_from(frame).map(Self::ReadCoils),
                function::READ_INPUTS => {
                    ReadDiscreteInputs::try_from(frame).map(Self::ReadDiscreteInputs)
                }
                function::READ_HOLDING_REGISTERS => {
                    ReadHoldingRegisters::try_from(frame).map(Self::ReadHolsingRegisters)
                }
                function::READ_INPUT_REGISTERS => {
                    ReadInputRegisters::try_from(frame).map(Self::ReadInputRegisters)
                }
                function::WRITE_COIL => WriteCoil::try_from(frame).map(Self::WriteCoil),
                function::WRITE_HOLDING_REGISTER => {
                    WriteHoldingRegister::try_from(frame).map(Self::WriteHoldingRegister)
                }
                function::WRITE_MULTIPLE_COILS => {
                    WriteMultipleCoils::try_from(frame).map(Self::WriteMultipleCoils)
                }
                function::WRITE_MULTIPLE_HOLDING_REGISTERS => {
                    WriteMultipleHoldingRegisters::try_from(frame)
                        .map(Self::WriteMultipleHoldingRegisters)
                }
                // unknwn function code
                _ => Err(Error::UnknownFunction),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ReadCoils<'a> {
        bytes: &'a [u8],
    }

    impl<'a> ReadCoils<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() >= Self::minimum_len().into(),
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn payload_len(&self) -> u8 {
            self.bytes[2]
        }

        pub fn iter_coils(&'_ self) -> impl Iterator<Item = bool> + '_ {
            let data = {
                // header(2) + location(2) + count(2) + payload_count(1)
                let start_idx = 3;
                let end_idx = self.bytes.len() - 2;
                &self.bytes[start_idx..end_idx]
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
            try_from_bytes(bytes)
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ReadDiscreteInputs<'a> {
        bytes: &'a [u8],
    }

    impl<'a> TryFrom<Frame<'a>> for ReadCoils<'a> {
        type Error = crate::Error;

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            if frame.function() != Self::FUNCTION {
                Err(Error::UnexpectedFunction)
            } else if !Self::is_valid_len(frame.raw_bytes().len()) {
                Err(Error::DecodeInvalidLength)
            } else {
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<ReadCoils<'a>> for Frame<'a> {
        fn from(command: ReadCoils<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }

    impl<'a> ReadDiscreteInputs<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() == 4 + 4,
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn payload_len(&self) -> u8 {
            self.bytes[2]
        }

        pub fn iter_inputs(&'_ self) -> impl Iterator<Item = bool> + '_ {
            self.bytes[3..]
                .windows(2)
                .map(byteorder::BigEndian::read_u16)
                .flat_map(|val| bitvec::array::BitArray::<[u16; 1], Msb0>::new([val]).into_iter())
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
        const FUNCTION: Function = function::READ_INPUTS;
    }

    impl<'a> TryFrom<&'a [u8]> for ReadDiscreteInputs<'a> {
        type Error = crate::Error;

        fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<ReadDiscreteInputs<'a>> for Frame<'a> {
        fn from(command: ReadDiscreteInputs<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ReadHoldingRegisters<'a> {
        bytes: &'a [u8],
    }

    impl<'a> ReadHoldingRegisters<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() == 4 + 4,
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn payload_len(&self) -> u8 {
            self.bytes[2]
        }

        pub fn iter_registers(&'_ self) -> impl Iterator<Item = u16> + '_ {
            self.bytes[3..]
                .windows(2)
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
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<ReadHoldingRegisters<'a>> for Frame<'a> {
        fn from(command: ReadHoldingRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ReadInputRegisters<'a> {
        bytes: &'a [u8],
    }

    impl<'a> ReadInputRegisters<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() == 4 + 4,
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn payload_len(&self) -> u8 {
            self.bytes[2]
        }

        pub fn iter_registers(&'_ self) -> impl Iterator<Item = u16> + '_ {
            self.bytes[3..]
                .windows(2)
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
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<ReadInputRegisters<'a>> for Frame<'a> {
        fn from(command: ReadInputRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct WriteMultipleCoils<'a> {
        bytes: &'a [u8],
    }

    impl<'a> WriteMultipleCoils<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() == 4 + 4,
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn start_index(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[2..])
        }

        pub fn register_count(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[4..])
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
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<WriteMultipleCoils<'a>> for Frame<'a> {
        fn from(command: WriteMultipleCoils<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct WriteMultipleHoldingRegisters<'a> {
        bytes: &'a [u8],
    }

    impl<'a> WriteMultipleHoldingRegisters<'a> {
        pub fn new(bytes: &'a [u8]) -> Self {
            debug_assert!(
                // 4 bytes for RTU, 4 bytes of payload
                bytes.len() == 4 + 4,
                "length validation should be done before constructing an instance"
            );
            Self { bytes }
        }

        pub fn start_index(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[2..])
        }

        pub fn register_count(&self) -> u16 {
            byteorder::BigEndian::read_u16(&self.bytes[4..])
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
            try_from_bytes(bytes)
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
                Ok(Self::new(frame.into_raw_bytes()))
            }
        }
    }

    impl<'a> From<WriteMultipleHoldingRegisters<'a>> for Frame<'a> {
        fn from(command: WriteMultipleHoldingRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new_unchecked(bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::function;

    use super::*;

    #[test]
    fn command_write_multiple_coils() {
        let mut buf = [0; 256];
        // 0xB, 0xF, 0x0, 0x1B, 0x0, 0x9, 0x2, 0x4D, 0x1, 0x6C, 0xA7
        let frame = crate::builder::build_frame(&mut buf)
            .for_address(0xB)
            .function(function::WRITE_MULTIPLE_COILS)
            .registers([27, 9])
            .byte(2)
            .bytes([0x4D, 0x01])
            .finalise();

        let command = command::WriteMultipleCoils::try_from(frame.clone()).unwrap();
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

    #[test]
    fn response_read_multiple_coils() {
        let mut buf = [0; 256];
        // 0xB, 0x1, 0x4, 0xCD, 0x6B, 0xB2, 0x7F, 0x2B, 0xE1
        let frame = crate::builder::build_frame(&mut buf)
            .for_address(0xB)
            .function(function::READ_COILS)
            .byte(4)
            .bytes([0xCD, 0x6B, 0xB2, 0x7F])
            .finalise();
        let command = response::ReadCoils::try_from(frame.clone()).unwrap();
        assert_eq!(command.payload_len(), 4);
        let coils = command.iter_coils().collect::<Vec<_>>();
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
