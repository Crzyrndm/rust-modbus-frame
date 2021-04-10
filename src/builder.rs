use core::marker::PhantomData;

use crate::{device::Device, exception, Function};

pub trait Encoding<Out> {
    /// writes the header of the frame, returns the initial offset
    /// return the next start idx
    fn init(&mut self);
    /// write a single bytes to the buffer, starting at idx
    /// return the next start idx
    fn write_bytes(&mut self, bytes: &[u8]);
    /// write a register value to the buffer starting at idx
    /// return the next start idx
    fn write_registers(&mut self, registers: &[u16]);
    /// write error check and any other trailing data
    fn finalise(self) -> Out;

    fn bytes_remaining(&self) -> usize;
    fn bytes_consumed(&self) -> usize;
}

pub struct Builder<Out, Encoder, STATE>
where
    Encoder: Encoding<Out>,
{
    // how to write to the buffer
    encoder: Encoder,
    // typestate (0-sized type to limit available functions)
    _state: STATE,
    // result type of calling encoder.finalise()
    _out: PhantomData<Out>,
}

/// initial state, only header written
pub struct Initial;
/// address set, function next
pub struct AddFunction;
/// add data, then finalise to a frame
pub struct AddData;

impl<Out, Encoder: Encoding<Out>> From<Encoder> for Builder<Out, Encoder, Initial> {
    fn from(enc: Encoder) -> Self {
        Builder {
            encoder: enc,
            _state: Initial {},
            _out: PhantomData,
        }
    }
}

/// following functions can be used in any state to check on the builder progress if neccesary
impl<ENCODING, Out, STATE> Builder<Out, ENCODING, STATE>
where
    ENCODING: Encoding<Out>,
{
    pub fn bytes_consumed(&self) -> usize {
        self.encoder.bytes_consumed()
    }

    pub fn bytes_remaining(&self) -> usize {
        self.encoder.bytes_remaining()
    }
}

impl<'b, ENCODING, Out> Builder<Out, ENCODING, Initial>
where
    ENCODING: Encoding<Out>,
{
    pub fn for_device(mut self, device: &Device) -> Builder<Out, ENCODING, AddFunction> {
        self.encoder.init();
        self.encoder.write_bytes(&[device.address()]);
        Builder {
            encoder: self.encoder,
            _state: AddFunction {},
            _out: PhantomData,
        }
    }
}

impl<Out, ENCODING> Builder<Out, ENCODING, AddFunction>
where
    ENCODING: Encoding<Out>,
{
    pub fn function(mut self, function: Function) -> Builder<Out, ENCODING, AddData> {
        self.encoder.write_bytes(&[function.0]);
        Builder {
            encoder: self.encoder,
            _state: AddData {},
            _out: PhantomData,
        }
    }

    pub fn exception(mut self, function: Function, exception: exception::Exception) -> Out {
        self.encoder.write_bytes(&[function.0 | 0x80, exception.0]);
        self.encoder.finalise()
    }
}

impl<Out, ENCODING> Builder<Out, ENCODING, AddData>
where
    ENCODING: Encoding<Out>,
{
    /// bytes copied directly into the frame data as is
    pub fn bytes(mut self, bytes: &[u8]) -> Builder<Out, ENCODING, AddData> {
        self.encoder.write_bytes(bytes);
        self
    }

    /// copied directly into the frame data as is
    pub fn byte(self, b: u8) -> Builder<Out, ENCODING, AddData> {
        self.bytes(&[b])
    }

    /// registers copied into the frame data as big endian bytes
    pub fn registers(mut self, registers: &[u16]) -> Builder<Out, ENCODING, AddData> {
        self.encoder.write_registers(registers);
        self
    }

    /// register copied into the frame data as big endian bytes
    pub fn register(self, r: u16) -> Builder<Out, ENCODING, AddData> {
        self.registers(&[r])
    }

    /// calculate CRC to finalise the frame
    pub fn finalise(self) -> Out {
        self.encoder.finalise()
    }
}
