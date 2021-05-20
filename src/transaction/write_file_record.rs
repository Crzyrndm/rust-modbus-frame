//! 0x15

use super::read_file_record::FileSubRequest;

pub struct Request<'a, 'b> {
    subrequests: &'b [(FileSubRequest, &'a [u16])],
}

/// response echos request
pub struct Response<'a, 'b>(Request<'a, 'b>);
