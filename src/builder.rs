//! construct a modbus frame structure in a provided buffer
//! internally, this uses the RTU format without the CRC

use crate::{device::Device, frame::Frame, Exception, Function};

#[derive(Debug)]
pub struct Builder<'b, STATE> {
    buffer: &'b mut [u8],
    idx: usize,
    // typestate (0-sized type to limit available functions)
    _state: STATE,
}

/// initial state, nothing set
pub struct Initial;
/// address set, function next
pub struct AddFunction;
/// add data, then finalise to a frame
pub struct AddData;

/// building frames conveniently
/// ```
/// use modbus_frames as modbus;
/// use modbus::device::Device;
/// use modbus::Function;
/// use modbus::builder;
///
/// let mut buff = [0u8; 20];
/// let frame = builder::build_frame(&mut buff)
///                 .for_device(Device::new(1))
///                 .function(Function(2))
///                 .register(3)
///                 .finalise();
/// assert_eq!(frame.raw_bytes(), [1, 2, 0, 3]);
/// ```
pub fn build_frame<'b>(buff: &'b mut [u8]) -> Builder<'b, Initial> {
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
    pub fn for_device(self, device: Device) -> Builder<'b, AddFunction> {
        self.buffer[self.idx] = device.address();
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

    pub fn exception(self, function: Function, exception: Exception) -> Frame<'b> {
        self.function(Function(function.0 | 0x80))
            .byte(exception.0)
            .finalise()
    }
}

impl<'b> Builder<'b, AddData> {
    /// bytes copied directly into the frame data as is
    pub fn bytes<I: Iterator<Item = u8>>(mut self, iter: I) -> Builder<'b, AddData> {
        for byte in iter {
            self.buffer[self.idx] = byte;
            self.idx += 1;
        }
        self
    }

    /// copied directly into the frame data as is
    pub fn byte(self, b: u8) -> Builder<'b, AddData> {
        self.bytes([b].iter().copied())
    }

    /// registers copied into the frame data as big endian bytes
    pub fn registers<I: Iterator<Item = u16>>(mut self, iter: I) -> Builder<'b, AddData> {
        for register in iter {
            let bytes = register.to_be_bytes();
            self.buffer[self.idx] = bytes[0];
            self.buffer[self.idx + 1] = bytes[1];

            self.idx += 2;
        }
        self
    }

    /// register copied into the frame data as big endian bytes
    pub fn register(self, r: u16) -> Builder<'b, AddData> {
        self.registers([r].iter().copied())
    }

    pub fn finalise(self) -> Frame<'b> {
        Frame::new(&self.buffer[..self.idx])
    }
}

#[cfg(test)]
mod tests {
    use super::build_frame;
    use crate::{device::Device, rtu::crc, Function};

    #[test]
    fn test_builder() {
        let mut buff = [0u8; 20];
        // address state
        let frame = build_frame(&mut buff);
        assert_eq!(0, frame.bytes_consumed());
        assert_eq!(20, frame.bytes_remaining());
        // function state
        let frame = frame.for_device(Device::new(123));
        assert_eq!(1, frame.bytes_consumed());
        assert_eq!(19, frame.bytes_remaining());
        // data state
        let frame = frame.function(Function(213));
        assert_eq!(2, frame.bytes_consumed());
        assert_eq!(18, frame.bytes_remaining());

        let frame = frame
            .byte(1)
            .register(4)
            .bytes([2, 3].iter().copied())
            .registers([5, 6].iter().copied());
        assert_eq!(11, frame.bytes_consumed());
        assert_eq!(9, frame.bytes_remaining());
        // as frame
        let frame = frame.finalise();
        assert_eq!(11, frame.raw_bytes().len());
        assert_eq!(Device::new(123), frame.device());
        assert_eq!(Function(213), frame.function());
        assert_eq!([1, 0, 4, 2, 3, 0, 5, 0, 6], frame.payload());

        let frame_crc = frame.crc();
        let crc = crc::calculate(&buff[..11]);
        assert_eq!(crc, frame_crc);
    }
}
