#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
// this crate is intended for use in both hosted and embedded contexts. No allocations or other conveniences

// pub once exists
pub mod ascii;
pub mod builder;
pub mod device;
pub mod entity;
pub mod error;
pub mod exception;
pub mod frame;
pub mod function;
pub mod iter;
pub mod modbus_traits;
pub mod rtu;
pub mod transaction;

type Result<T> = core::result::Result<T, error::Error>;

/// function code specifies how a device processes the frame
/// top bit is set to indicate an exception response so valid range is 0-127
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Function(pub u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Exception(pub u8);
