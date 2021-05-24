#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Device {
    adr: u8,
}

impl Device {
    pub const fn new(address: u8) -> Self {
        Device { adr: address }
    }

    pub const fn address(&self) -> u8 {
        self.adr
    }
}

#[cfg(test)]
mod tests {
    use super::Device;

    #[test]
    fn test_new_device() {
        for address in 1..255 {
            let device_1 = Device::new(address);
            assert_eq!(device_1.adr, address);
            assert_eq!(device_1.address(), address);
        }
    }
}
