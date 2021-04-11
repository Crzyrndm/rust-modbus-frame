use crate::ascii::frame::Frame;
use crate::builder::Encoding;

/// Modbus ASCII references
/// - https://modbus.org/docs/Modbus_Application_Protocol_V1_1b.pdf
/// - https://www.virtual-serial-port.org/articles/modbus-ascii-guide/
pub struct ASCII<'b> {
    buffer: &'b mut [u8],
    /// lrc sum is calculated on the data bytes before they are formatted to ASCII
    /// since we are progressively encoding, need to track the sum
    lrc_sum: u8,
    idx: usize,
}

impl<'b> ASCII<'b> {
    pub fn new(buffer: &'b mut [u8]) -> ASCII<'b> {
        ASCII {
            buffer,
            lrc_sum: 0,
            idx: 0,
        }
    }

    /// write byte as 2 hex chars, upper nibble first from current idx
    /// increments idx
    fn write_hex(&mut self, byte: u8) {
        self.lrc_sum = self.lrc_sum.wrapping_add(byte);
        self.buffer[self.idx] = match (byte & 0xF0) >> 4 {
            val @ (0..=9) => b'0' + val,
            other => b'A' + other - 10,
        };
        self.buffer[self.idx + 1] = match byte & 0x0F {
            val @ (0..=9) => b'0' + val,
            other => b'A' + other - 10,
        };
        self.idx += 2;
    }
}

impl<'b> Encoding<Frame<'b>> for ASCII<'b> {
    fn init(&mut self) {
        // ASCII start delimiter is ':' / 0x3A
        self.buffer[0] = b':';
        self.idx = 1;
        self.lrc_sum = 0;
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_hex(byte);
        }
    }

    fn write_registers(&mut self, registers: &[u16]) {
        // registers need to be written in big endian
        for r in registers {
            let bytes = r.to_be_bytes();
            self.write_hex(bytes[0]);
            self.write_hex(bytes[1]);
        }
    }

    fn finalise(mut self) -> Frame<'b> {
        // write the LRC bytes
        let lrc = (!self.lrc_sum).wrapping_add(1);
        self.write_hex(lrc);
        // and the '\r\n'
        self.buffer[self.idx] = b'\r';
        self.buffer[self.idx + 1] = b'\n';
        // return the frame
        let len = self.idx + 2;
        Frame::new(&self.buffer[..len])
    }

    fn bytes_remaining(&self) -> usize {
        self.buffer.len() - self.idx
    }

    fn bytes_consumed(&self) -> usize {
        self.idx
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::Encoding;

    use super::ASCII;
    #[test]
    fn test_write_hex() {
        let mut buffer = [0u8; 10];
        let mut ascii = ASCII::new(&mut buffer[..]);
        ascii.write_hex(0x7A);

        assert_eq!(ascii.buffer[0], b'7');
        assert_eq!(ascii.buffer[1], b'A');
        assert_eq!(ascii.idx, 2);
    }

    #[test]
    fn test_encode() {
        let mut buffer = [0u8; 20];
        let mut ascii = ASCII::new(&mut buffer[..]);
        ascii.init();
        ascii.write_bytes(&[0x12, 0x34]);
        ascii.write_registers(&[0x5678]);
        let frame = ascii.finalise();

        assert_eq!(frame.raw_bytes(), b":12345678EC\r\n");
    }
}
