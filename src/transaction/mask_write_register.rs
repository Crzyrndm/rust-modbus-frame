//! 0x16

/// Result = (Current Contents AND And_Mask) OR (Or_Mask AND (NOT And_Mask))
pub struct Request {
    address: u16,
    and_mask: u16,
    or_mask: u16,
}

// response?
