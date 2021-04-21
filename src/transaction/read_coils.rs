/// 0x01
pub struct Request {
    address: u16,
    quantity: u16,
}

/// 0x01
pub struct Response<'b> {
    bitmap: &'b [u8],
}
