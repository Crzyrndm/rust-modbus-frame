//! 0x17

use super::read_holding_registers;

/// read and write in a single transaction
/// write operation is completed before read begins
pub struct Request<'b> {
    read_address: u16,
    read_quantity: u16,
    write_address: u16,
    write_quantity: u16,
    write_values: &'b [u16],
}

/// response is standard read registers
pub struct Response<'a>(read_holding_registers::Response<'a>);
