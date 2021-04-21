//! 0x10

use super::write_multiple_coils;

/// success response echos the address/quantity
pub struct WriteMultipleRegisters<'b> {
    address: u16,
    quantity: u16,
    registers: &'b [u16],
}

/// success response echos the address/quantity
pub struct Response(write_multiple_coils::Response);
