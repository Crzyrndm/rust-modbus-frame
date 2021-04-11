#[derive(PartialEq, Debug, Clone)]
#[non_exhaustive] // new errors may be added later
pub enum Error {
    None,
    InvalidLength,
    InvalidCorrupt,
    InvalidEncoding,
    OtherAddress,
    WrongFunction,
}

// std::error::Error trait obviously isn't available in no_std
// whould this implement any other error traits?
