//! 0x04

use super::read_holding_registers;

pub struct Request(read_holding_registers::Request);

pub struct ReadInputRegisters<'b>(read_holding_registers::Response<'b>);
