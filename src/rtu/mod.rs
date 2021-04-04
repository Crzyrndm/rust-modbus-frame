pub mod crc;
pub mod defines;
pub mod device;
pub mod frame;
pub mod view;

#[derive(PartialEq, Debug, Clone)]
#[non_exhaustive] // new errors may be added later
pub enum Errors {
    None,
    TooShort,
    TooLong,
    OtherAddress,
    WrongFunction,
    InvalidCrC,
}
