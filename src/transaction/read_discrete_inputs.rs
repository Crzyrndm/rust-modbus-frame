//! 0x02

use super::read_coils;

pub struct Request(read_coils::Request);
pub struct Response<'b>(read_coils::Response<'b>);
