//! 0x06
//!

pub struct Request {
    address: u16,
    new_value: u16,
}

/// Success response is an echo of the command
pub struct Response(Request);
