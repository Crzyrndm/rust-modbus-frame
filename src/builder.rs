//! construct a modbus frame structure in a provided buffer
//! internally, this uses the RTU format without the CRC

use core::ops::Rem;

use byteorder::ByteOrder;

use crate::{calculate_crc16, frame::Frame, Exception, Function};

/// Write modbus messages more conveniently and coherently using named operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Builder<'b, STATE> {
    buffer: &'b mut [u8],
    idx: usize,
    // typestate (0-sized type to limit available functions)
    _state: STATE,
}

/// Builder state tag type
/// initial state, nothing set
pub struct Initial;
/// /// Builder state tag type
/// address set, function next
pub struct AddFunction;
/// Builder state tag type
/// add data, then finalise to a frame
pub struct AddData;

/// building frames conveniently
/// ```
/// use modbus_frames::{builder, Function};
///
/// let mut buff = [0u8; 20];
/// let (frame, rem) = builder::build_frame(&mut buff)
///                 .for_address(1)
///                 .function(Function(2))
///                 .register(3)
///                 .finalise();
/// assert_eq!(frame.raw_bytes(), [1, 2, 0, 3, 224, 25]);
/// ```
pub fn build_frame(buff: &'_ mut [u8]) -> Builder<'_, Initial> {
    Builder {
        buffer: buff,
        idx: 0,
        _state: Initial {},
    }
}

/// following functions can be used in any state to check on the builder progress if neccesary
impl<'b, STATE> Builder<'b, STATE> {
    pub fn state(&'b self) -> &'b [u8] {
        &self.buffer[..self.idx]
    }

    pub fn bytes_consumed(&self) -> usize {
        self.idx
    }

    pub fn bytes_remaining(&self) -> usize {
        self.buffer.len() - self.idx
    }
}

impl<'b> Builder<'b, Initial> {
    pub fn for_address(self, address: u8) -> Builder<'b, AddFunction> {
        self.buffer[self.idx] = address;
        Builder {
            buffer: self.buffer,
            idx: 1,
            _state: AddFunction {},
        }
    }
}

impl<'b> Builder<'b, AddFunction> {
    pub fn function(self, function: Function) -> Builder<'b, AddData> {
        self.buffer[self.idx] = function.0;
        Builder {
            buffer: self.buffer,
            idx: 2,
            _state: AddData {},
        }
    }

    pub fn exception(self, function: Function, exception: Exception) -> (Frame<'b>, &'b mut [u8]) {
        self.function(Function(function.0 | 0x80))
            .byte(exception.0)
            .finalise()
    }
}

impl<'b> Builder<'b, AddData> {
    /// bytes copied directly into the frame data as is
    pub fn bytes<I: IntoIterator<Item = u8>>(mut self, iter: I) -> Builder<'b, AddData> {
        for byte in iter {
            self.buffer[self.idx] = byte;
            self.idx += 1;
        }
        self
    }

    /// bytes copied directly into the frame data as is
    pub fn byte(mut self, b: u8) -> Builder<'b, AddData> {
        self.buffer[self.idx] = b;
        self.idx += 1;
        self
    }

    /// bits are packed into bytes, first bit in LSB
    /// returns the updated builder and the number of bits actually written which is otherwise not
    /// discoverable (can't tell how many trailing `false` values were present)
    pub fn bits(mut self, bits: impl IntoIterator<Item = bool>) -> (Builder<'b, AddData>, usize) {
        // LSB is the addressed coil with following addresses in order
        let mut b = 0;
        let mut bit_count = 0;
        for bit in bits {
            let bit_idx = bit_count.rem(u8::BITS as usize);
            if bit {
                b |= 1 << bit_idx;
            }
            if u8::BITS as usize == bit_idx + 1 {
                self = self.byte(b);
                b = 0;
            }
            bit_count += 1
        }
        if bit_count.rem(u8::BITS as usize) != 0 {
            self = self.byte(b);
        }

        (self, bit_count)
    }

    /// registers copied into the frame data as big endian bytes
    pub fn registers<I: IntoIterator<Item = u16>>(mut self, iter: I) -> Builder<'b, AddData> {
        for register in iter {
            byteorder::BigEndian::write_u16(&mut self.buffer[self.idx..], register);
            self.idx += 2;
        }
        self
    }

    /// register copied into the frame data as big endian bytes
    pub fn register(self, r: u16) -> Builder<'b, AddData> {
        self.registers([r].iter().copied())
    }

    pub fn count_following_bytes(mut self, to_count: impl FnOnce(Self) -> Self) -> Self {
        let current_idx = self.idx;
        self.idx += 1;
        let builder = to_count(self);
        // -1 because we're not counting the marker byte
        let count = (builder.idx - current_idx - 1) as u8;
        builder.buffer[current_idx] = count;
        builder
    }

    /// This formats a set of registers as [register_count (u16), byte count (u8), registers... ([u16])]
    /// This would be used in e.g. "WriteMultipleHoldingRegisters" to avoid double iteration of the registers source
    pub fn count_registers(mut self, registers: impl IntoIterator<Item = u16>) -> Self {
        let current_idx = self.idx;
        self.idx += 2;
        let builder = self.count_following_bytes(|builder| builder.registers(registers));

        let count = builder.buffer[current_idx + 2] as u16 / 2; // num bytes written / 2
        byteorder::BigEndian::write_u16(&mut builder.buffer[current_idx..], count);
        builder
    }

    /// This formats a set of registers as [register_count (u16), byte count (u8), registers... ([u16])]
    /// This would be used in e.g. "WriteMultipleCoils" to avoid double iteration of the bits source
    pub fn count_bits(mut self, bits: impl IntoIterator<Item = bool>) -> Self {
        let current_idx = self.idx;
        self.idx += 2;

        self.count_following_bytes(|builder| {
            let (builder, bit_count) = builder.bits(bits);
            byteorder::BigEndian::write_u16(&mut builder.buffer[current_idx..], bit_count as u16);
            builder
        })
    }

    pub fn finalise(self) -> (Frame<'b>, &'b mut [u8]) {
        let crc = calculate_crc16(&self.buffer[..self.idx]);
        byteorder::LittleEndian::write_u16(&mut self.buffer[self.idx..], crc);
        let (frame, remainder) = self.buffer.split_at_mut(self.idx + 2);
        (Frame::new_unchecked(frame), remainder)
    }
}

#[cfg(test)]
mod tests {
    use byteorder::ByteOrder;

    use super::build_frame;
    use crate::{calculate_crc16, Function};

    #[test]
    fn test_builder() {
        let mut buff = [0u8; 20];
        // address state
        let frame = build_frame(&mut buff);
        assert_eq!(0, frame.bytes_consumed());
        assert_eq!(20, frame.bytes_remaining());
        // function state
        let frame = frame.for_address(123);
        assert_eq!(1, frame.bytes_consumed());
        assert_eq!(19, frame.bytes_remaining());
        // data state
        let frame = frame.function(Function(213));
        assert_eq!(2, frame.bytes_consumed());
        assert_eq!(18, frame.bytes_remaining());

        let frame = frame.count_following_bytes(|builder| {
            builder
                .byte(1)
                .register(4)
                .bytes([2, 3].iter().copied())
                .registers([5, 6].iter().copied())
        });
        assert_eq!(12, frame.bytes_consumed());
        assert_eq!(8, frame.bytes_remaining());
        // as frame
        let (frame, _remainder) = frame.finalise();
        assert_eq!(14, frame.raw_bytes().len());
        assert_eq!(123, frame.address());
        assert_eq!(Function(213), frame.function());
        assert_eq!([9, 1, 0, 4, 2, 3, 0, 5, 0, 6], frame.payload());

        let frame_crc = frame.calculate_crc();
        let crc = calculate_crc16(&buff[..12]);
        assert_eq!(crc, frame_crc);
    }

    #[test]
    fn count_registers() {
        let mut buff = [0; 20];
        let registers = [1, 2, 3];
        let (frame, _remainder) = build_frame(&mut buff)
            .for_address(1)
            .function(Function(1))
            .register(0)
            .count_registers(registers)
            .finalise();
        // 2 for address, 2 for count, 1 for byte count, 2x3 for values
        assert_eq!(frame.payload().len(), 11);
        assert_eq!(frame.payload()[4], 6); // 6 bytes
        assert_eq!(frame.payload()[2..4], [0, 3]); // 3 registers

        let mut decoded = [0; 3];
        byteorder::BigEndian::read_u16_into(&frame.payload()[5..11], &mut decoded);
        assert_eq!(decoded, registers);
    }

    #[test]
    fn count_bits() {
        let mut buff = [0; 20];
        let encoding = [0x62, 0xA];
        let (frame, _remainder) = build_frame(&mut buff)
            .for_address(1)
            .function(Function(1))
            .register(0)
            .count_bits(
                encoding
                    .iter()
                    .flat_map(|b| {
                        // byte to bits (LSB first)
                        [
                            b & 0x01 == 0x01,
                            b & 0x02 == 0x02,
                            b & 0x04 == 0x04,
                            b & 0x08 == 0x08,
                            b & 0x10 == 0x10,
                            b & 0x20 == 0x20,
                            b & 0x40 == 0x40,
                            b & 0x80 == 0x80,
                        ]
                    })
                    .take(10),
            )
            .finalise();
        // 2 for address, 2 for count, 1 for byte count, 2 for values
        assert_eq!(frame.payload().len(), 7);
        assert_eq!(frame.payload()[4], 2); // 2 bytes
        assert_eq!(frame.payload()[2..4], [0, 10]); // 10 bits
        assert_eq!(frame.payload()[5..7], [0x62, 0x02]);
    }
}
