//! 0x05
//!

use super::write_register;

pub struct Request(write_register::Request);

/// Success response is an echo of the command
pub struct Response(Request);
