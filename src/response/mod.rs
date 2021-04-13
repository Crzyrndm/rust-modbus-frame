/// 0x01
pub struct ReadCoils<'b> {
    bitmap: &'b [u8],
}

/// 0x02
pub struct ReadDiscreteInputs<'b> {
    bitmap: &'b [u8],
}

/// 0x03
pub struct ReadHoldingRegisters<'b> {
    registers: &'b [u16],
}

/// 0x04
pub struct ReadInputRegisters<'b> {
    registers: &'b [u16],
}

/// 0x05
/// Success response is an echo of the command
pub struct WriteCoil {
    address: u16,
    new_state: u16,
}

/// 0x06
/// Success response is an echo of the command
pub struct WriteRegister {
    address: u16,
    new_value: u16,
}

/// 0x0F
/// success response echos the address/quantity
pub struct WriteMultipleCoils {
    address: u16,
    quantity: u16,
}

/// 0x10
/// success response echos the address/quantity
pub struct WriteMultipleRegisters {
    address: u16,
    quantity: u16,
}
