pub fn calculate(bytes: &[u8]) -> u16 {
    crc16::State::<crc16::MODBUS>::calculate(bytes)
}
