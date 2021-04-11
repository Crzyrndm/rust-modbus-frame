/*!
README examples duplicated here for testing. Better way?

## RTU frame from bytes example
```rust
use modbus_frames as modbus;
use std::convert::TryFrom;
use modbus::frame;
use modbus::device::Device;
use modbus::function;

// incoming bytes
let bytes: &[u8] = &[0x11, 0x03, 0x00, 0x6B, 0x00, 0x03, 0x76, 0x87];
// try_from checks that the length is within modbus allowances (4 <= len <= 255)
// and that the crc is valid.
// frame::Frame is a borrow of the slice providing named accesor functions  for the bytes within
if let Ok(frame) = frame::Frame::try_from(bytes) {
    assert_eq!(frame.device(), Device::new(0x11));
    assert_eq!(frame.function(), function::READ_HOLDING_REGISTERS);
    assert_eq!(frame.payload(), [0x00, 0x6B, 0x00, 0x03]);
    assert_eq!(frame.crc().to_le_bytes(), [0x76, 0x87]);
    // and since no copies were made, a view of the original bytes is available (excluding CRC)
    assert_eq!(frame.raw_bytes(), &bytes[..(bytes.len() - 2)]);
}
```

## RTU frame builder example
```rust
use modbus_frames as modbus;
use modbus::builder;
use modbus::device::Device;
use modbus::function;

// creating the above bytes using the frame builder
let mut buffer = [0; 10];
// frame builder uses typestates to ensure that the frame can only be built
// in the correct order (address, function, data, crc)
let frame = builder::build_frame(&mut buffer)
                    .for_device(&Device::new(0x11))
                    .function(function::READ_HOLDING_REGISTERS)
                    .bytes(&[0x00, 0x6B])
                    .register(0x03)
                    .finalise();

assert_eq!(frame.device(), Device::new(0x11));
assert_eq!(frame.function(), function::READ_HOLDING_REGISTERS);
assert_eq!(frame.payload(), [0x00, 0x6B, 0x00, 0x03]);
assert_eq!(frame.crc().to_le_bytes(), [0x76, 0x87]);
```
*/

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
pub mod rtu;
pub mod view;

type Result<T> = core::result::Result<T, error::Error>;

/// function code specifies how a device processes the frame
/// top bit is set to indicate an exception response so valid range is 0-127
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Function(pub u8);

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Exception(pub u8);
