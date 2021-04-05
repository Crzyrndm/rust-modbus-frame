use crate::{device::Device, error, rtu::crc, Function, Result};
use core::convert::{TryFrom, TryInto};

/// Frame provides functions to view a series of bytes as a modbus data frame
#[derive(PartialEq, Debug, Clone)]
pub struct Frame<'b> {
    data: &'b [u8],
}

impl<'b> Frame<'b> {
    /// Assumes validation occurs prior to call as no guarantees are checked here
    /// in particular, this will lead to panics if bytes.len() is less than 2
    /// requirements for this to be safe
    /// - buffer size is in the inclusive range [4:255]
    /// - the last two bytes are the correct modbus crc (crc::calculate()) for the preceding bytes
    ///
    /// The safe alternative depend on data source.
    /// - &[u8] -> use Frame::try_from, invalid length or CRC will result in an error
    /// - frame::build_frame will construct a valid frame from various components in a reasonably ergonomix form
    ///
    /// new_unchecked is primarily intended for use in implementing (Try)Into for custom types where TryFrom is unnecesary
    pub unsafe fn new_unchecked(bytes: &'b [u8]) -> Self {
        Frame { data: bytes }
    }

    pub fn address(&self) -> u8 {
        self.data[0]
    }

    pub fn function(&self) -> Function {
        self.data[1].into()
    }

    pub fn crc(&self) -> u16 {
        let data_len = self.data.len() - 2;
        u16::from_le_bytes(self.data[data_len..].try_into().unwrap())
    }

    pub fn payload(&self) -> &[u8] {
        let data_len = self.data.len() - 2;
        &self.data[2..data_len]
    }

    pub fn raw_bytes(&self) -> &[u8] {
        &self.data
    }
}

/// a Frame has minimal requirements:
/// - be atleast 4 bytes. ANything less cannot exist
/// - have a valid CRC as the final 2 bytes
///
/// Commands and responses may impose tighter restrictions, but the raw frame is left open
/// (e.g. all of the standard frames require the length be <= 255)
pub fn validate(bytes: &[u8]) -> error::Error {
    if bytes.len() < 4 {
        // incomplete / invalid data packet
        return error::Error::InvalidLength;
    }
    let data_len = bytes.len() - 2;
    let msg_crc = crc::calculate(&bytes[0..data_len]);
    if bytes[data_len..] != msg_crc.to_le_bytes() {
        return error::Error::InvalidCrC; // invalid crc
    }
    return error::Error::None;
}

impl<'b> TryFrom<&'b [u8]> for Frame<'b> {
    type Error = error::Error;

    fn try_from(bytes: &'b [u8]) -> Result<Self> {
        match validate(bytes) {
            Self::Error::None => Ok(Frame { data: bytes }),
            other => Err(other),
        }
    }
}

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
/// use modbus::rtu::frame::build_frame;
/// use modbus::device::Device;
/// use modbus::Function;
/// let mut buff = [0u8; 20];
/// let frame = build_frame(&mut buff)
///                 .for_device(&Device::new(1))
///                 .function(Function(2))
///                 .register(3)
///                 .to_frame();
/// assert_eq!(frame.raw_bytes(), [1, 2, 0, 3, 224, 25]);
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
    pub fn for_device(self, device: &Device) -> Builder<'b, AddFunction> {
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
}

impl<'b> Builder<'b, AddData> {
    /// bytes copied directly into the frame data as is
    pub fn bytes(mut self, b: &[u8]) -> Builder<'b, AddData> {
        b.iter()
            .enumerate()
            .for_each(|(i, b)| self.buffer[i + self.idx] = *b);
        self.idx += b.len();
        self
    }

    /// copied directly into the frame data as is
    pub fn byte(self, b: u8) -> Builder<'b, AddData> {
        self.bytes(&[b])
    }

    /// registers copied into the frame data as big endian bytes
    pub fn registers(mut self, r: &[u16]) -> Builder<'b, AddData> {
        r.iter().enumerate().for_each(|(i, r)| {
            let bytes = r.to_be_bytes();
            self.buffer[self.idx + 2 * i] = bytes[0];
            self.buffer[self.idx + 2 * i + 1] = bytes[1];
        });
        self.idx += 2 * r.len();
        self
    }

    /// register copied into the frame data as big endian bytes
    pub fn register(self, r: u16) -> Builder<'b, AddData> {
        self.registers(&[r])
    }

    /// calculate CRC to finalise the frame
    pub fn to_frame(self) -> Frame<'b> {
        let crc = crc::calculate(&self.buffer[..self.idx]).to_le_bytes();
        self.buffer[self.idx] = crc[0];
        self.buffer[self.idx + 1] = crc[1];

        let len = self.idx + 2;
        unsafe { Frame::new_unchecked(&self.buffer[..len]) }
    }
}

#[cfg(test)]
mod tests {
    use super::build_frame;
    use crate::{
        device::Device,
        rtu::{crc, frame::Frame},
        Function,
    };

    #[test]
    fn test_frame_views() {
        let test_data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        // this frame is *definitely* invalid (bad crc)
        // only used because this is a dead simple test asserting the accesor functions work
        // getting the crc right only obscures the purpose
        let frame = unsafe { Frame::new_unchecked(&test_data[..]) };

        assert_eq!(frame.address(), 0);
        assert_eq!(frame.function(), Function(1));
        assert_eq!(frame.payload(), [2, 3, 4, 5, 6, 7]);
        assert_eq!(frame.crc().to_le_bytes(), [8, 9]);
    }

    #[test]
    fn test_builder() {
        let mut buff = [0u8; 20];
        // address state
        let frame = build_frame(&mut buff);
        assert_eq!(0, frame.bytes_consumed());
        assert_eq!(20, frame.bytes_remaining());
        assert_eq!(0, frame.state().len());
        // function state
        let frame = frame.for_device(&Device::new(123));
        assert_eq!(1, frame.bytes_consumed());
        assert_eq!(19, frame.bytes_remaining());
        assert_eq!([123], frame.state());
        // data state
        let frame = frame.function(Function(213));
        assert_eq!(2, frame.bytes_consumed());
        assert_eq!(18, frame.bytes_remaining());
        assert_eq!([123, 213], frame.state());

        let frame = frame.byte(1).register(4).bytes(&[2, 3]).registers(&[5, 6]);
        assert_eq!(11, frame.bytes_consumed());
        assert_eq!(9, frame.bytes_remaining());
        assert_eq!([123, 213, 1, 0, 4, 2, 3, 0, 5, 0, 6], frame.state());
        // as frame
        let frame = frame.to_frame();
        assert_eq!(13, frame.raw_bytes().len());
        assert_eq!(123, frame.address());
        assert_eq!(Function(213), frame.function());
        assert_eq!([1, 0, 4, 2, 3, 0, 5, 0, 6], frame.payload());

        let frame_crc = frame.crc();
        let crc = crc::calculate(&buff[..11]);
        assert_eq!(crc, frame_crc);
    }

    #[test]
    fn test_device_validate_msg() {
        use super::validate;
        use crate::error;

        const MAX_LEN: usize = 300;
        const TEST_ADDRESS: u8 = 2;

        let mut msg_buffer = [0u8; MAX_LEN];
        for len in 0..MAX_LEN {
            let msg = &mut msg_buffer[0..len];
            match len {
                0..=3 => {
                    // too short, no point doing address/crc shenanigans
                    assert!(error::Error::InvalidLength == validate(msg));
                }
                _ => {
                    msg[0] = TEST_ADDRESS; // valid address
                    msg[1] = len as u8; // function cade == len
                    msg[2..] // fill the rest with incrementing numbers
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, v)| *v = i as u8);

                    assert!(error::Error::InvalidCrC == validate(msg));
                    // calculate actual crc
                    let data_len = msg.len() - 2;
                    let msg_crc_bytes = crc::calculate(&msg[0..data_len]).to_le_bytes();
                    msg[data_len] = msg_crc_bytes[0];
                    msg[data_len + 1] = msg_crc_bytes[1];
                    assert!(error::Error::None == validate(msg));
                }
            }
        }
    }
}
