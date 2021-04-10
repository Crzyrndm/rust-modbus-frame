/// https://en.wikipedia.org/wiki/Longitudinal_redundancy_check
/// calculation is very simple
/// 1. (sum all bytes) modulo 256
/// 2. ((invert bits of SUM) modulo 256 + 1) modulo 256
pub fn calculate(bytes: &[u8]) -> u8 {
    let mut sum: u8 = 0;
    for &byte in bytes {
        sum = sum.wrapping_add(byte);
    }
    (!sum).wrapping_add(1)
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_lrc() {
        let data = [247, 3, 19, 137, 0, 10];
        let lrc = calculate(&data);
        assert_eq!(96, lrc);
    }
}
