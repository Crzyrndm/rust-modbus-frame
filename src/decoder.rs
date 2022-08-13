//! Take bytes, turn into outputs

use byteorder::ByteOrder;

use crate::{frame::Frame, function, Error, FixedLen, Function, FunctionCode, PacketLen};

/// When Writing/Reading a single coil, `ON == 0xFF00` and `OFF == 0x0000`
/// All other values are invalid
pub const COIL_ON: u16 = 0xFF00;
/// When Writing/Reading a single coil, `ON == 0xFF00` and `OFF == 0x0000`
/// All other values are invalid
pub const COIL_OFF: u16 = 0x0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteCoil<'a> {
    frame: Frame<'a>,
}

impl<'a> WriteCoil<'a> {
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
        byteorder::BigEndian::read_u16(&self.frame.payload()[2..]) == COIL_ON
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
pub struct WriteHoldingRegister<'a> {
    frame: Frame<'a>,
}

impl<'a> WriteHoldingRegister<'a> {
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

pub mod command {
    use bitvec::macros::internal::funty::Fundamental;
    use bitvec::order::Msb0;
    use byteorder::ByteOrder;

    use crate::frame::Frame;
    use crate::{function, Error, FixedLen, Function, FunctionCode, PacketLen};

    pub use super::{WriteCoil, WriteHoldingRegister};

    /// The default responses for a decode type
    /// ```
    /// use modbus_frames::{builder, function, decoder::command::CommonCommands};
    /// # let mut buf = [0; 256];
    /// let command_frame = builder::build_frame(&mut buf)
    ///                        .for_address(0x11)
    ///                        .function(function::READ_COILS)
    ///                        .registers([0x13, 0x25])
    ///                        .finalise();
    /// let decoded = CommonCommands::try_from(command_frame).unwrap();
    /// assert!(matches!(decoded, CommonCommands::ReadCoils(_)));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    impl<'a> CommonCommands<'a> {
        pub fn as_frame(&self) -> Frame {
            (*self).into()
        }
    }

    impl<'a> From<CommonCommands<'a>> for Frame<'a> {
        fn from(response: CommonCommands<'a>) -> Self {
            match response {
                CommonCommands::ReadCoils(res) => res.as_frame(),
                CommonCommands::ReadDiscreteInputs(res) => res.as_frame(),
                CommonCommands::ReadHolsingRegisters(res) => res.as_frame(),
                CommonCommands::ReadInputRegisters(res) => res.as_frame(),
                CommonCommands::WriteCoil(res) => res.as_frame(),
                CommonCommands::WriteHoldingRegister(res) => res.as_frame(),
                CommonCommands::WriteMultipleCoils(res) => res.as_frame(),
                CommonCommands::WriteMultipleHoldingRegisters(res) => res.as_frame(),
            }
        }
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
                function::READ_DISCRETE_INPUTS => {
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ReadCoils<'a> {
        frame: Frame<'a>,
    }

    impl<'a> ReadCoils<'a> {
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
    pub struct ReadDiscreteInputs<'a> {
        frame: Frame<'a>,
    }

    impl<'a> ReadDiscreteInputs<'a> {
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
    pub struct ReadHoldingRegisters<'a> {
        frame: Frame<'a>,
    }

    impl<'a> ReadHoldingRegisters<'a> {
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
    pub struct ReadInputRegisters<'a> {
        frame: Frame<'a>,
    }

    impl<'a> ReadInputRegisters<'a> {
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
    pub struct WriteMultipleCoils<'a> {
        frame: Frame<'a>,
    }

    impl<'a> WriteMultipleCoils<'a> {
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
    pub struct WriteMultipleHoldingRegisters<'a> {
        frame: Frame<'a>,
    }

    impl<'a> WriteMultipleHoldingRegisters<'a> {
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
}

pub mod response {
    use bitvec::order::Lsb0;
    use byteorder::ByteOrder;

    pub use super::{WriteCoil, WriteHoldingRegister};

    use crate::{frame::Frame, function, Error, FixedLen, Function, FunctionCode, PacketLen};

    /// The default responses for a decode type
    /// ```
    /// use modbus_frames::{builder, function, decoder::response::CommonResponses};
    /// # let mut buf = [0; 256];
    /// let response_frame = builder::build_frame(&mut buf)
    ///            .for_address(0xB)
    ///            .function(function::READ_COILS)
    ///            .byte(4)
    ///            .bytes([0xCD, 0x6B, 0xB2, 0x7F])
    ///            .finalise();
    /// let decoded = CommonResponses::try_from(response_frame).unwrap();
    /// assert!(matches!(decoded, CommonResponses::ReadCoils(_)));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
                function::READ_COILS => ReadCoils::try_from(frame).map(Self::ReadCoils),
                function::READ_DISCRETE_INPUTS => {
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ReadCoils<'a> {
        frame: Frame<'a>,
    }

    impl<'a> ReadCoils<'a> {
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
    pub struct ReadDiscreteInputs<'a> {
        frame: Frame<'a>,
    }

    impl<'a> ReadDiscreteInputs<'a> {
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
    pub struct ReadHoldingRegisters<'a> {
        frame: Frame<'a>,
    }

    impl<'a> ReadHoldingRegisters<'a> {
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
    pub struct ReadInputRegisters<'a> {
        frame: Frame<'a>,
    }

    impl<'a> ReadInputRegisters<'a> {
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
    pub struct WriteMultipleCoils<'a> {
        frame: Frame<'a>,
    }

    impl<'a> WriteMultipleCoils<'a> {
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
    pub struct WriteMultipleHoldingRegisters<'a> {
        frame: Frame<'a>,
    }

    impl<'a> WriteMultipleHoldingRegisters<'a> {
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
}

#[cfg(test)]
mod tests {
    use crate::{decoder::response::CommonResponses, function};

    use super::{command::CommonCommands, *};

    #[test]
    fn command_read_coils() {
        let mut buf = [0; 256];
        // 11 01 0013 0025 0E84
        let frame = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::READ_COILS)
            .registers([0x13, 0x25])
            .finalise();

        let commands = [
            command::ReadCoils::try_from(frame.raw_bytes()).unwrap(),
            command::ReadCoils::try_from(frame).unwrap(),
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
        let frame = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::READ_DISCRETE_INPUTS)
            .registers([0xC4, 0x16])
            .finalise();

        let commands = [
            command::ReadDiscreteInputs::try_from(frame.raw_bytes()).unwrap(),
            command::ReadDiscreteInputs::try_from(frame).unwrap(),
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
        let frame = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::READ_HOLDING_REGISTERS)
            .registers([0x6B, 3])
            .finalise();

        let commands = [
            command::ReadHoldingRegisters::try_from(frame.raw_bytes()).unwrap(),
            command::ReadHoldingRegisters::try_from(frame).unwrap(),
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
        let frame = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::READ_INPUT_REGISTERS)
            .registers([8, 1])
            .finalise();

        let commands = [
            command::ReadInputRegisters::try_from(frame.raw_bytes()).unwrap(),
            command::ReadInputRegisters::try_from(frame).unwrap(),
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
        let frame = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::WRITE_COIL)
            .registers([0xAC, COIL_ON])
            .finalise();

        let commands = [
            command::WriteCoil::try_from(frame.raw_bytes()).unwrap(),
            command::WriteCoil::try_from(frame).unwrap(),
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
        let frame = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::WRITE_HOLDING_REGISTER)
            .registers([1, 3])
            .finalise();

        let commands = [
            command::WriteHoldingRegister::try_from(frame.raw_bytes()).unwrap(),
            command::WriteHoldingRegister::try_from(frame).unwrap(),
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
        let frame = crate::builder::build_frame(&mut buf)
            .for_address(0xB)
            .function(function::WRITE_MULTIPLE_COILS)
            .registers([27, 9])
            .byte(2)
            .bytes([0x4D, 0x01])
            .finalise();

        let commands = [
            command::WriteMultipleCoils::try_from(frame.raw_bytes()).unwrap(),
            command::WriteMultipleCoils::try_from(frame).unwrap(),
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
        let frame = crate::builder::build_frame(&mut buf)
            .for_address(0x11)
            .function(function::WRITE_MULTIPLE_HOLDING_REGISTERS)
            .registers([1, 2])
            .byte(4)
            .registers([0xA, 0x0102])
            .finalise();

        let commands = [
            command::WriteMultipleHoldingRegisters::try_from(frame.raw_bytes()).unwrap(),
            command::WriteMultipleHoldingRegisters::try_from(frame).unwrap(),
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
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_DISCRETE_INPUTS)
                .registers([0xC4, 0x16])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_HOLDING_REGISTERS)
                .registers([0x6B, 3])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_INPUT_REGISTERS)
                .registers([8, 1])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_COIL)
                .registers([0xAC, COIL_ON])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_HOLDING_REGISTER)
                .registers([1, 3])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0xB)
                .function(function::WRITE_MULTIPLE_COILS)
                .registers([27, 9])
                .byte(2)
                .bytes([0x4D, 0x01])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_MULTIPLE_HOLDING_REGISTERS)
                .registers([1, 2])
                .byte(4)
                .registers([0xA, 0x0102])
                .finalise()
                .raw_bytes()
                .to_vec(),
        ];
        let result = commands
            .into_iter()
            .map(|bytes| {
                let byte = CommonCommands::try_from(bytes.as_slice()).unwrap();
                let frame = Frame::new_unchecked(&bytes);
                let frame = CommonCommands::try_from(frame).unwrap();
                [format!("{:?}", byte), format!("{:?}", frame)]
            })
            .collect::<Vec<_>>();
        dbg!(result);
    }

    #[test]
    fn response_read_coils() {
        let mut buf = [0; 256];
        // 0xB, 0x1, 0x4, 0xCD, 0x6B, 0xB2, 0x7F, 0x2B, 0xE1
        let frame = crate::builder::build_frame(&mut buf)
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

        let frame = crate::builder::build_frame(&mut buf)
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
        let frame = crate::builder::build_frame(&mut buf)
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
        let frame = crate::builder::build_frame(&mut buf)
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
        let frame = crate::builder::build_frame(&mut buf)
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
        let frame = crate::builder::build_frame(&mut buf)
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
        let frame = crate::builder::build_frame(&mut buf)
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
        let frame = crate::builder::build_frame(&mut buf)
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
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0xB)
                .function(function::READ_DISCRETE_INPUTS)
                .byte(4)
                .bytes([0xCD, 0x6B, 0xB2, 0x7F])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_HOLDING_REGISTERS)
                .byte(6)
                .registers([0xAE41, 0x5652, 0x4340])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::READ_INPUT_REGISTERS)
                .byte(6)
                .registers([0xAE41, 0x5652, 0x4340])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_COIL)
                .registers([0xAC, COIL_ON])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_HOLDING_REGISTER)
                .registers([1, 3])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0xB)
                .function(function::WRITE_MULTIPLE_COILS)
                .registers([27, 9])
                .finalise()
                .raw_bytes()
                .to_vec(),
            crate::builder::build_frame(&mut buf)
                .for_address(0x11)
                .function(function::WRITE_MULTIPLE_HOLDING_REGISTERS)
                .registers([1, 2])
                .finalise()
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
