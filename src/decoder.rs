//! Take bytes, turn into outputs

use byteorder::ByteOrder;

use crate::{frame::Frame, FixedLen, PacketLen};

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

impl<'a> TryFrom<&'a [u8]> for WriteCoil<'a> {
    type Error = ();

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() == Self::minimum_len().into() {
            // TODO: Validate coil value
            Ok(Self::new(value))
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<Frame<'a>> for WriteCoil<'a> {
    type Error = ();

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        let bytes = frame.into_raw_bytes();
        Self::try_from(bytes)
    }
}

impl<'a> From<WriteCoil<'a>> for Frame<'a> {
    fn from(command: WriteCoil<'_>) -> Frame<'_> {
        let bytes = command.bytes;
        Frame::new(bytes)
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

impl<'a> TryFrom<&'a [u8]> for WriteHoldingRegister<'a> {
    type Error = ();

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() == Self::minimum_len().into() {
            Ok(Self::new(value))
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<Frame<'a>> for WriteHoldingRegister<'a> {
    type Error = ();

    fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
        let bytes = frame.into_raw_bytes();
        Self::try_from(bytes)
    }
}

impl<'a> From<WriteHoldingRegister<'a>> for Frame<'a> {
    fn from(command: WriteHoldingRegister<'_>) -> Frame<'_> {
        let bytes = command.bytes;
        Frame::new(bytes)
    }
}

pub mod command {
    use bitvec::macros::internal::funty::Fundamental;
    use bitvec::order::Msb0;
    use byteorder::ByteOrder;

    use crate::frame::Frame;
    use crate::{FixedLen, PacketLen};

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
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            match value[1] {
                1 => ReadCoils::try_from(value).map(Self::ReadCoils),
                2 => ReadDiscreteInputs::try_from(value).map(Self::ReadDiscreteInputs),
                3 => ReadHoldingRegisters::try_from(value).map(Self::ReadHolsingRegisters),
                4 => ReadInputRegisters::try_from(value).map(Self::ReadInputRegisters),
                5 => WriteCoil::try_from(value).map(Self::WriteCoil),
                6 => WriteHoldingRegister::try_from(value).map(Self::WriteHoldingRegister),
                15 => WriteMultipleCoils::try_from(value).map(Self::WriteMultipleCoils),
                16 => WriteMultipleHoldingRegisters::try_from(value)
                    .map(Self::WriteMultipleHoldingRegisters),

                _ => Err(()),
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

    impl<'a> TryFrom<&'a [u8]> for ReadCoils<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() == Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for ReadCoils<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<ReadCoils<'a>> for Frame<'a> {
        fn from(command: ReadCoils<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
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

    impl<'a> TryFrom<&'a [u8]> for ReadDiscreteInputs<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() == Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for ReadDiscreteInputs<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<ReadDiscreteInputs<'a>> for Frame<'a> {
        fn from(command: ReadDiscreteInputs<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
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

    impl<'a> TryFrom<&'a [u8]> for ReadHoldingRegisters<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() == Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for ReadHoldingRegisters<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<ReadHoldingRegisters<'a>> for Frame<'a> {
        fn from(command: ReadHoldingRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
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

    impl<'a> TryFrom<&'a [u8]> for ReadInputRegisters<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() == Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for ReadInputRegisters<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<ReadInputRegisters<'a>> for Frame<'a> {
        fn from(command: ReadInputRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
        }
    }

    use super::WriteCoil;
    use super::WriteHoldingRegister;

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

    impl<'a> TryFrom<&'a [u8]> for WriteMultipleCoils<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() >= Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for WriteMultipleCoils<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<WriteMultipleCoils<'a>> for Frame<'a> {
        fn from(command: WriteMultipleCoils<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
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

    impl<'a> TryFrom<&'a [u8]> for WriteMultipleHoldingRegisters<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() >= Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for WriteMultipleHoldingRegisters<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<WriteMultipleHoldingRegisters<'a>> for Frame<'a> {
        fn from(command: WriteMultipleHoldingRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
        }
    }
}

pub mod response {
    use bitvec::order::Lsb0;
    use bitvec::order::Msb0;
    use byteorder::ByteOrder;

    use crate::frame::Frame;
    use crate::{FixedLen, PacketLen};

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
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            match value[1] {
                1 => ReadCoils::try_from(value).map(Self::ReadCoils),
                2 => ReadDiscreteInputs::try_from(value).map(Self::ReadDiscreteInputs),
                3 => ReadHoldingRegisters::try_from(value).map(Self::ReadHolsingRegisters),
                4 => ReadInputRegisters::try_from(value).map(Self::ReadInputRegisters),
                5 => WriteCoil::try_from(value).map(Self::WriteCoil),
                6 => WriteHoldingRegister::try_from(value).map(Self::WriteHoldingRegister),
                15 => WriteMultipleCoils::try_from(value).map(Self::WriteMultipleCoils),
                16 => WriteMultipleHoldingRegisters::try_from(value)
                    .map(Self::WriteMultipleHoldingRegisters),

                _ => Err(()),
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

    impl<'a> TryFrom<&'a [u8]> for ReadCoils<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() >= Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct ReadDiscreteInputs<'a> {
        bytes: &'a [u8],
    }

    impl<'a> TryFrom<Frame<'a>> for ReadCoils<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<ReadCoils<'a>> for Frame<'a> {
        fn from(command: ReadCoils<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
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

    impl<'a> TryFrom<&'a [u8]> for ReadDiscreteInputs<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() >= Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for ReadDiscreteInputs<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<ReadDiscreteInputs<'a>> for Frame<'a> {
        fn from(command: ReadDiscreteInputs<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
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

    impl<'a> TryFrom<&'a [u8]> for ReadHoldingRegisters<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() >= Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for ReadHoldingRegisters<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<ReadHoldingRegisters<'a>> for Frame<'a> {
        fn from(command: ReadHoldingRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
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

    impl<'a> TryFrom<&'a [u8]> for ReadInputRegisters<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() >= Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for ReadInputRegisters<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<ReadInputRegisters<'a>> for Frame<'a> {
        fn from(command: ReadInputRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
        }
    }

    use super::WriteCoil;
    use super::WriteHoldingRegister;

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

    impl<'a> TryFrom<&'a [u8]> for WriteMultipleCoils<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() == Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for WriteMultipleCoils<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<WriteMultipleCoils<'a>> for Frame<'a> {
        fn from(command: WriteMultipleCoils<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
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

    impl<'a> TryFrom<&'a [u8]> for WriteMultipleHoldingRegisters<'a> {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() == Self::minimum_len().into() {
                Ok(Self::new(value))
            } else {
                Err(())
            }
        }
    }

    impl<'a> TryFrom<Frame<'a>> for WriteMultipleHoldingRegisters<'a> {
        type Error = ();

        fn try_from(frame: Frame<'a>) -> Result<Self, Self::Error> {
            let bytes = frame.into_raw_bytes();
            Self::try_from(bytes)
        }
    }

    impl<'a> From<WriteMultipleHoldingRegisters<'a>> for Frame<'a> {
        fn from(command: WriteMultipleHoldingRegisters<'_>) -> Frame<'_> {
            let bytes = command.bytes;
            Frame::new(bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_write_multiple_coils() {
        let raw = [
            0xB_u8, 0xF, 0x00, 0x1B, 0x00, 0x09, 0x02, 0x4D, 0x01, 0x6C, 0xA7,
        ];
        let command = command::WriteMultipleCoils::try_from(raw.as_slice()).unwrap();
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
        let raw = [0x0B_u8, 0x01, 0x04, 0xCD, 0x6B, 0xB2, 0x7F, 0x2B, 0xE1];
        let command = response::ReadCoils::try_from(raw.as_slice()).unwrap();
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
