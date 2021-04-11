use core::usize;

use crate::frame;

use super::crc;

pub struct AsBytesIter<'b> {
    frame: frame::Frame<'b>,
    // calculated once idx exceeds frame.raw_bytes().len()
    crc: Option<[u8; 2]>,
    idx: usize,
}

impl<'b> AsBytesIter<'b> {
    pub fn new(frame: frame::Frame<'b>) -> AsBytesIter<'b> {
        AsBytesIter {
            frame,
            crc: None,
            idx: 0,
        }
    }
}

impl<'b> Iterator for AsBytesIter<'b> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let out = if self.idx < self.frame.raw_bytes().len() {
            Some(self.frame.raw_bytes()[self.idx])
        } else if let None = self.crc {
            // self.idx == self.frame.raw_bytes().len()
            self.crc = Some(crc::calculate(self.frame.raw_bytes()).to_le_bytes());
            Some(self.crc.unwrap()[0])
        } else if self.idx == self.frame.raw_bytes().len() + 1 {
            Some(self.crc.unwrap()[1])
        } else {
            None
        };
        self.idx += 1;
        out
    }
}
