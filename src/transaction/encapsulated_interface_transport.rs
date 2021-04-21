//! 0x2B

pub struct Request<Data> {
    mei_type: u8,
    mei_data: Data,
}

/// mei_type = 0x0D (13d)?
pub struct CanOpenPdu<'b> {
    can_data: &'b [u8],
}

/// mei_type = 0x0E (14d)?
pub struct ReadDeviceId {
    // 1 = basic, 2 = regular, 3 = extended, 4 = device specific
    read_id_code: u8,
    // the data id
    object_id: u8,
}

// response(s)?
