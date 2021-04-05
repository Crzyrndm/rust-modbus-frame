# Why another modbus crate
You'd be correct in thinking that the modbus protocol being both simple and not obscure, someone has written a rust crate to handle it. Sure enough, there are a few on crates.io
- [modbus](https://docs.rs/modbus/modbus/)
- [tokio-modbus](https://crates.io/crates/tokio-modbus)
- [rmodbus](https://crates.io/crates/rmodbus)
- [modbus-iiot](https://crates.io/crates/modbus-iiot)
- [and so on](https://crates.io/search?q=modbus&sort=relevance)

There's two reasons for *this* crate to exist

1) my requirements are quite specific
   - no_std and small memory footprint
     - I'm an embedded developer. Wasteful FLASH/RAM usage and dynamic allocations are out
   - Needs to support custom / non-standard function codes (e.g. 0x46).
     - Some of the devices I want to communicate with use more than just the standard register and coil commands, so libraries exposing just 'read_holding_registers' and co are not going to cut it
2) Probably more importantly though, it's a way to learn more Rust in an application I'm quite familiar with and have a use for

# About
This crate is about providing the building blocks for device communicating using the modbus protocol (which boils down to a header and crc wrapping some data). This is represented by the Frame struct

## RTU frame from bytes example
```rust
use std::convert::TryFrom;
use modbus::rtu::frame;
use modbus::device::Device;
use modbus::standard::function;

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
    // and since no copies were made, a view of the original bytes is available
    assert_eq!(frame.raw_bytes(), bytes);
}
```

## RTU frame builder example
```rust
use modbus::rtu::frame;
use modbus::device::Device;
use modbus::standard::function;

// creating the above bytes using the frame builder
let mut buffer = [0; 10];
// frame builder uses typestates to ensure that the frame can only be built
// in the correct order (address, function, data, crc)
let frame: frame::Frame = frame::build_frame(&mut buffer)
                                    .for_device(&Device::new(0x11))
                                    .function(function::READ_HOLDING_REGISTERS)
                                    .bytes(&[0x00, 0x6B])
                                    .register(0x03)
                                    .to_frame();

assert_eq!(frame.device(), Device::new(0x11));
assert_eq!(frame.function(), function::READ_HOLDING_REGISTERS);
assert_eq!(frame.payload(), [0x00, 0x6B, 0x00, 0x03]);
assert_eq!(frame.crc().to_le_bytes(), [0x76, 0x87]);
```