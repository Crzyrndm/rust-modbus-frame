//! 0x04

use super::read_holding_registers;

pub struct Request<'b>(read_holding_registers::Request<'b>);

pub struct ReadInputRegisters<'b>(read_holding_registers::Response<'b>);
