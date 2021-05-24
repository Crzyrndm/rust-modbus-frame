use crate::frame;

/// Modbus ASCII references
/// - https://modbus.org/docs/Modbus_Application_Protocol_V1_1b.pdf
/// - https://www.virtual-serial-port.org/articles/modbus-ascii-guide/

fn hex_nibble(nibble: u8) -> u8 {
    match nibble {
        val @ (0x0..=0x9) => b'0' + val,
        val @ (0xA..=0xF) => b'A' + val - 10,
        _ => panic!("input isn't a nibble"),
    }
}

fn high_nibble(byte: u8) -> u8 {
    byte >> 4
}

fn low_nibble(byte: u8) -> u8 {
    byte & 0x0F
}

pub struct AsBytesIter<'b> {
    frame: frame::Frame<'b>,
    // calculated once idx exceeds 1 + 2 * frame.raw_bytes().len()
    lrc: u8,
    idx: usize,
}

impl<'b> AsBytesIter<'b> {
    pub fn new(frame: frame::Frame<'b>) -> AsBytesIter<'b> {
        AsBytesIter {
            frame,
            lrc: 0,
            idx: 0,
        }
    }
}

impl<'b> Iterator for AsBytesIter<'b> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let out = if self.idx == 0 {
            Some(b':')
        } else {
            let ascii_raw_bytes_end = self.frame.raw_bytes().len() * 2 + 1;
            if self.idx < ascii_raw_bytes_end {
                let byte_idx = (self.idx - 1) / 2;
                let byte = self.frame.raw_bytes()[byte_idx];
                // high nibble first
                let hex = match (self.idx - 1) % 2 {
                    0 => hex_nibble(high_nibble(byte)),
                    _ => hex_nibble(low_nibble(byte)),
                };
                Some(hex)
            } else {
                match self.idx - ascii_raw_bytes_end {
                    0 => {
                        self.lrc = super::lrc::calculate(self.frame.raw_bytes());
                        Some(hex_nibble(high_nibble(self.lrc)))
                    }
                    1 => Some(hex_nibble(low_nibble(self.lrc))),
                    2 => Some(b'\r'),
                    3 => Some(b'\n'),
                    _ => None,
                }
            }
        };
        self.idx += 1;
        out
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.frame.raw_bytes().len() + 5;
        (len, Some(len))
    }
}

#[cfg(test)]
mod tests {
    use super::AsBytesIter;
    use crate::{builder, device::Device, function};

    #[test]
    fn test_ascii_as_bytes() {
        let mut buffer = [0; 20];
        let frame = builder::build_frame(&mut buffer)
            .for_device(Device::new(0xF7))
            .function(function::READ_HOLDING_REGISTERS)
            .bytes(&[19, 137, 0, 10])
            .finalise();

        let iter = AsBytesIter::new(frame);
        let bytes: Vec<_> = iter.collect();
        assert_eq!(bytes, b":F7031389000A60\r\n");
    }
}
