//! 0x03

pub struct Request {
    address: u16,
    quantity: u16,
}

pub struct Response<'b> {
    registers: &'b [u16],
}
