//! 0x0F

/// success response echos the address/quantity
pub struct WriteMultipleCoils<'b> {
    address: u16,
    quantity: u16,
    /// 1 = ON, 0 = OFF
    /// bytes are ordered from high to low address
    /// MSB of each byte has the highest address, extend with 0 bits
    bitmap: &'b [u8],
}

/// success response echos the address/quantity
pub struct Response {
    address: u16,
    quantity: u16,
}
