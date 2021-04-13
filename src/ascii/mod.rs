pub mod iterator;
pub mod lrc;

pub use iterator::AsBytesIter;

use crate::{error, frame::Frame, Result};

fn hex_to_nibble(hex: u8) -> u8 {
    match hex {
        val @ b'0'..=b'9' => val - b'0',
        val @ b'A'..=b'F' => val - b'A' + 0xA,
        _ => panic!("not expected hex range"),
    }
}

fn to_upper_nibble(nibble: u8) -> u8 {
    nibble << 4
}

/// validate buffer, transform hex to bytes and return a frame
/// in case of error, buffer is returned
pub fn decode(bytes: &mut [u8]) -> (Result<Frame>, &mut [u8]) {
    // 3 bytes framing, 2 each for address, function, lrc
    if bytes.len() < 9 {
        return (Err(error::Error::InvalidLength), bytes);
    }
    if bytes[0] != b':' {
        return (Err(error::Error::InvalidEncoding), bytes);
    } else if &bytes[(bytes.len() - 2)..] != &[b'\r', b'\n'] {
        return (Err(error::Error::InvalidEncoding), bytes);
    }
    let end_idx = (bytes.len() - 3) / 2;
    for idx in 0..end_idx {
        let hex_idx = 1 + 2 * idx;
        bytes[idx] =
            to_upper_nibble(hex_to_nibble(bytes[hex_idx])) | hex_to_nibble(bytes[hex_idx + 1]);
    }
    if lrc::calculate(&bytes[..(end_idx - 1)]) != bytes[end_idx - 1] {
        return (Err(error::Error::InvalidCorrupt), bytes);
    }
    let slices = bytes.split_at_mut(end_idx - 1);
    (Ok(Frame::new(slices.0)), slices.1)
}

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn test_ascii_decode() {
        let mut ascii = [
            b':', b'F', b'7', b'0', b'3', b'1', b'3', b'8', b'9', b'0', b'0', b'0', b'A', b'6',
            b'0', b'\r', b'\n',
        ];
        let expected_len = 6;
        let expexted_rem = ascii.len() - expected_len;
        let (decoded, remainder) = decode(&mut ascii);
        assert_eq!(remainder.len(), expexted_rem);

        let decoded = decoded.unwrap();
        assert_eq!(decoded.raw_bytes(), &[0xF7, 0x3, 0x13, 0x89, 0x0, 0xA]);
    }
}
