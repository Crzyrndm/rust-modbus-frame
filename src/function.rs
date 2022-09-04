//! function codes as documented by https://en.wikipedia.org/wiki/Modbus#Available_function/command_codes

/// function code specifies how a device processes the frame
/// top bit is set to indicate an exception response so valid range is 0-127
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Function(pub u8);

impl From<u8> for Function {
    fn from(f: u8) -> Self {
        Function(f)
    }
}

impl From<Function> for u8 {
    fn from(f: Function) -> Self {
        f.0
    }
}

/// Request:
///     Address of first coil to read (16-bit)
///     Number of coils to read (16-bit)
/// Value of each coil/discrete input is binary (0 for off, 1 for on). First requested coil/discrete input is stored as least significant bit of first byte in reply.
/// If number of coils/discrete inputs is not a multiple of 8, most significant bit(s) of last byte will be stuffed with zeros.
///
/// Normal response:
///     Number of bytes of coil values to follow (8-bit)
///     Coil values (8 coils/discrete inputs per byte)
pub const READ_COILS: Function = Function(1);

/// Request:
///     Address of first discrete input to read (16-bit)
///     Number of discrete inputs to read (16-bit)
/// Value of each coil/discrete input is binary (0 for off, 1 for on). First requested coil/discrete input is stored as least significant bit of first byte in reply.
/// If number of coils/discrete inputs is not a multiple of 8, most significant bit(s) of last byte will be stuffed with zeros.
///
/// Normal response:
///     Number of bytes of discrete input values to follow (8-bit)
///     Coil/discrete input values (8 discrete inputs per byte)
pub const READ_DISCRETE_INPUTS: Function = Function(2);

/// Request:
///    Address of first register to read (16-bit)
///    Number of registers to read (16-bit)
///
/// Normal response:
///    Number of bytes of register values to follow (8-bit)
///    Register values (16 bits per register)
pub const READ_HOLDING_REGISTERS: Function = Function(3);

/// Request:
///    Address of first register to read (16-bit)
///    Number of registers to read (16-bit)
///
/// Normal response:
///    Number of bytes of register values to follow (8-bit)
///    Register values (16 bits per register)
pub const READ_INPUT_REGISTERS: Function = Function(4);

/// Request:
///     Address of coil (16-bit)
///     Value to force/write: 0 for off and 65,280 (FF00 in hexadecimal) for on
///
/// Normal response: same as request.
pub const WRITE_COIL: Function = Function(5);

/// Request:
///    Address of holding register to preset/write (16-bit)
///    New value of the holding register (16-bit)
///
/// Normal response: same as request.
pub const WRITE_HOLDING_REGISTER: Function = Function(6);

/// Request:
///    Address of first coil to force/write (16-bit)
///    Number of coils to force/write (16-bit)
///    Number of bytes of coil values to follow (8-bit)
///    Coil values (8 coil values per byte)
/// Value of each coil is binary (0 for off, 1 for on). First requested coil is stored as least significant bit of first byte in request.
/// If number of coils is not a multiple of 8, most significant bit(s) of last byte should be stuffed with zeros. See example for function codes 1 and 2.
///
/// Normal response:
///    Address of first coil (16-bit)
///    number of coils (16-bit)
pub const WRITE_MULTIPLE_COILS: Function = Function(15);

/// Request:
///    Address of first holding register to preset/write (16-bit)
///    Number of holding registers to preset/write (16-bit)
///    Number of bytes of register values to follow (8-bit)
///    New values of holding registers (16 bits per register)
///
/// Normal response:
///    Address of first preset/written holding register (16-bit)
///    Number of preset/written holding registers (16-bit)
pub const WRITE_MULTIPLE_HOLDING_REGISTERS: Function = Function(16);

// pub const MASK_WRITE_REGISTER: Function = Function(22);
// pub const READ_WRITE_MULTIPLE_REGISTERS: Function = Function(23);
// pub const READ_FIFO_QUEUE: Function = Function(23);

// //diagnostics
// pub const READ_EXCEPTION_STATUS: Function = Function(7);
// pub const DIAGNOSTIC: Function = Function(8);
// pub const GET_COMM_EVENT_COUNTER: Function = Function(11);
// pub const GET_COM_EVENT_LOG: Function = Function(12);
// pub const REPORT_SLAVE_ID: Function = Function(17);
// pub const READ_DEVICE_ID: Function = Function(43);

// // file access
// pub const READ_FILE: Function = Function(20);
// pub const WRITE_FILE: Function = Function(21);
