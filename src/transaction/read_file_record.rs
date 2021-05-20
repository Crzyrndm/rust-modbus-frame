//! 0x14

pub struct FileSubRequest {
    reference_type: u8,
    file_number: u16,
    record_number: u16,
    record_length: u16,
}

pub struct Request<'b> {
    subrequests: &'b [FileSubRequest],
}
