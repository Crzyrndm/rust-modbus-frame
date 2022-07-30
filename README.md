# modbus-frames

## A simple modbus RTU library

By default this library knows about Coils (1/5/15), Discrete Inputs(2), Holding Registers(3/6/16), and Input Registers(4) only
Users can extend this by replacing the encoder implementation

### Decode

Takes in a slice of bytes, does basic validation (length/crc) then passes to the decoder
Decoder returns an enum (e.g. ReadHoldingRegisters(address, func, num_regs, &[regs], crc)) which the application can then act upon
Decoding doesn't require any copies to be made. Only references into the byte array

```rust
// TODO: decode example
```

### Encode

Encoding is done using the builder pattern. The builder is initialised with the scratch array, which it then fills in as
address, function, etc. are provided

```rust
use modbus_frames::{builder, Function};

let mut buff = [0u8; 20];
let frame = builder::build_frame(&mut buff)
                .for_address(1)
                .function(Function(2))
                .register(3)
                .finalise();
assert_eq!(frame.raw_bytes(), [1, 2, 0, 3, 224, 25]);
assert_eq!(frame.payload(), [0, 3]);
```
