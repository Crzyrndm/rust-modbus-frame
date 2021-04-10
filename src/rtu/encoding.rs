use crate::builder::Encoding;
use crate::rtu::{crc, frame};

pub struct RTU<'b> {
    buffer: &'b mut [u8],
    idx: usize,
}

impl<'b> RTU<'b> {
    pub fn new(buffer: &'b mut [u8]) -> RTU<'b> {
        RTU { buffer, idx: 0 }
    }
}

impl<'b> Encoding<frame::Frame<'b>> for RTU<'b> {
    fn init(&mut self) {
        // RTU needs no init
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        // bytes get written as is
        let end_idx = self.idx + bytes.len();
        self.buffer[self.idx..end_idx].copy_from_slice(bytes);
        self.idx += bytes.len();
    }

    fn write_registers(&mut self, registers: &[u16]) {
        // registers need to be written in big endian
        registers.iter().enumerate().for_each(|(i, r)| {
            let bytes = r.to_be_bytes();
            self.buffer[self.idx + 2 * i] = bytes[0];
            self.buffer[self.idx + 2 * i + 1] = bytes[1];
        });
        self.idx += 2 * registers.len();
    }

    fn finalise(self) -> frame::Frame<'b> {
        // write the CRC bytes, return a frame
        let crc = crc::calculate(&self.buffer[..self.idx]).to_le_bytes();
        self.buffer[self.idx] = crc[0];
        self.buffer[self.idx + 1] = crc[1];

        let len = self.idx + 2;
        unsafe { frame::Frame::new_unchecked(&self.buffer[..len]) }
    }

    fn bytes_remaining(&self) -> usize {
        self.buffer.len() - self.idx
    }

    fn bytes_consumed(&self) -> usize {
        self.idx
    }
}
