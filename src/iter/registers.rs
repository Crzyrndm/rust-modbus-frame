pub struct Registers<'a> {
    payload_iter: core::slice::Iter<'a, u8>,
}

impl<'a> Registers<'a> {
    /// shouldn't be used directly, instead payloads provide an iterate method which implements this
    pub fn create(payload: &'a [u8]) -> Registers<'a> {
        Registers {
            payload_iter: payload.iter(),
        }
    }
}

impl<'a> Iterator for Registers<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let high = self.payload_iter.next();
        if let Some(&high) = high {
            let low = self.payload_iter.next();
            if let Some(&low) = low {
                return Some(((high as u16) << 8) | low as u16);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Registers;

    #[test]
    fn test_register_iter() {
        let buff = [0x00, 0x03, 0x51, 0x87, 0x20, 0x75, 0x71];
        let iter = Registers::create(&buff[..]);
        let regs: Vec<_> = iter.collect();
        // series of u16 from bytes (big endian). overflow bytes ignored
        assert_eq!(regs, [0x0003, 0x5187, 0x2075]);
    }
}
