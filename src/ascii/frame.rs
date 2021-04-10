use crate::{device::Device, Function};

pub struct Frame<'b> {
    data: &'b [u8],
}

fn from_hex(inp: &[u8]) -> u8 {
    assert!(inp.len() == 2);
    let high = match inp[0] {
        val @ (b'0'..=b'9') => val - b'0',
        other => 10 + other - b'A',
    };
    let low = match inp[1] {
        val @ (b'0'..=b'9') => val - b'0',
        other => 10 + other - b'A',
    };
    (high << 4) | low
}

impl<'b> Frame<'b> {
    pub fn new(buffer: &[u8]) -> Frame {
        Frame { data: buffer }
    }

    pub fn device(&self) -> Device {
        Device::new(from_hex(&self.data[1..3]))
    }

    pub fn function(&self) -> Function {
        Function(from_hex(&self.data[3..5]))
    }

    pub fn lrc(&self) -> u8 {
        // last two bytes are '\r\n'
        // two bytes prior are the lrc
        let lrc_idx = self.data.len() - 4;
        let lrc = &self.data[lrc_idx..lrc_idx + 2];
        from_hex(lrc)
    }

    pub fn payload(&self) -> &[u8] {
        let lrc_idx = self.data.len() - 4;
        &self.data[5..lrc_idx]
    }

    pub fn raw_bytes(&self) -> &[u8] {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::from_hex;

    #[test]
    fn test_from_hex() {
        let inp = b"1D";
        assert_eq!(0x1D, from_hex(inp));
    }
}
