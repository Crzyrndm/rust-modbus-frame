//! defines for standard RTU
//! e.g. functions, exceptions, timeouts
//!

/// function codes as documented by https://en.wikipedia.org/wiki/Modbus#Available_function/command_codes
pub mod function {
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    pub struct Function(pub u8);
    impl From<u8> for Function {
        fn from(b: u8) -> Self {
            Function(b)
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
    pub const READ_INPUTS: Function = Function(2);

    /// Request:
    ///     Address of coil (16-bit)
    ///     Value to force/write: 0 for off and 65,280 (FF00 in hexadecimal) for on
    ///
    /// Normal response: same as request.
    pub const WRITE_COIL: Function = Function(5);

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
    ///    Address of holding register to preset/write (16-bit)
    ///    New value of the holding register (16-bit)
    ///
    /// Normal response: same as request.
    pub const WRITE_HOLDING_REGISTER: Function = Function(6);

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
    pub const MASK_WRITE_REGISTER: Function = Function(22);
    pub const READ_WRITE_MULTIPLE_REGISTERS: Function = Function(23);
    pub const READ_FIFO_QUEUE: Function = Function(23);

    //diagnostics
    pub const READ_EXCEPTION_STATUS: Function = Function(7);
    pub const DIAGNOSTIC: Function = Function(8);
    pub const GET_COMM_EVENT_COUNTER: Function = Function(11);
    pub const GET_COM_EVENT_LOG: Function = Function(12);
    pub const REPORT_SLAVE_ID: Function = Function(17);
    pub const READ_DEVICE_ID: Function = Function(43);

    // file access
    pub const READ_FILE: Function = Function(20);
    pub const WRITE_FILE: Function = Function(21);
}

// Exception codes as documented by https://en.wikipedia.org/wiki/Modbus#Exception_responses
pub mod exception {
    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    pub struct Exception(pub u8);
    impl From<u8> for Exception {
        fn from(b: u8) -> Self {
            Exception(b)
        }
    }

    /// Function code received in the query is not recognized or allowed by slave
    pub const ILLEGAL_FUNCTION: Exception = Exception(1);
    /// Data address of some or all the required entities are not allowed or do not exist in slave
    pub const ILLEGAL_ADDRESS: Exception = Exception(2); //
    /// Value is not accepted by slave
    pub const ILLEGAL_DATA: Exception = Exception(3);
    /// Unrecoverable error occurred while slave was attempting to perform requested action
    pub const DEBICE_FAILURE: Exception = Exception(4);
    /// Slave has accepted request and is processing it, but a long duration of time is required.
    /// This response is returned to prevent a timeout error from occurring in the master.
    /// Master can next issue a Poll Program Complete message to determine whether processing is completed
    pub const ACKNOWLEDGE: Exception = Exception(5);
    /// Slave is engaged in processing a long-duration command. Master should retry later
    pub const DEVICE_BUSY: Exception = Exception(6);
    /// Slave cannot perform the programming functions. Master should request diagnostic or error information from slave
    pub const NEGATIVE_ACKNOWLEDGE: Exception = Exception(7);
    /// Slave detected a parity error in memory. Master can retry the request, but service may be required on the slave device
    pub const MEMORY_PARITY_ERROR: Exception = Exception(8);
    /// Specialized for Modbus gateways. Indicates a misconfigured gateway
    pub const GATEWAY_PATH_UNAVAILABLE: Exception = Exception(10);
    /// Specialized for Modbus gateways. Sent when slave fails to respond
    pub const GATEWAY_DEVICE_NO_RESPONSE: Exception = Exception(11);
}

/// entity numbers are the long form ids of inputs/coils/registers
/// always 5 or 6 digits
///
/// The first digit identifies the type:
/// - 0 => coil
/// - 1 => discrete input
/// - 3 => input register
/// - 4 => holding register
///
/// The remaining digits identify the location in a 1-indexed format
/// - NOTE: address is 0-indexed
///
/// examples:
/// - 30001 -> input register at address 99
/// - 30100 -> input register at address 99
/// - 40001 -> holding register at address 0
/// - 40100 -> holding register at address 99
pub mod entity {
    use core::convert::TryFrom;

    #[derive(Debug, PartialEq, PartialOrd, Clone)]
    pub struct Entity<'a>(pub &'a str);
    impl<'a> From<&'a str> for Entity<'a> {
        fn from(s: &'a str) -> Self {
            Entity(s)
        }
    }

    pub enum EntityType {
        Coil,
        DiscreteInput,
        InputRegister = 3,
        HoldingRegister,
    }

    impl<'a> TryFrom<&Entity<'a>> for EntityType {
        type Error = &'static str;

        fn try_from(value: &Entity) -> Result<Self, Self::Error> {
            if value.0.len() != 5 && value.0.len() != 6 {
                return Err("invalid entity length");
            }
            if let Some(c) = value.0.chars().nth(0) {
                return match c {
                    '0' => Ok(EntityType::Coil),
                    '1' => Ok(EntityType::DiscreteInput),
                    '3' => Ok(EntityType::InputRegister),
                    '4' => Ok(EntityType::HoldingRegister),
                    _ => Err("first character was not a known entity type"),
                };
            }
            unreachable!("length already checked");
        }
    }

    impl<'a> TryFrom<&Entity<'a>> for u16 {
        type Error = &'static str;

        fn try_from(entity: &Entity) -> Result<u16, Self::Error> {
            if entity.0.len() != 5 && entity.0.len() != 6 {
                return Err("invalid entity length");
            }
            let address_str = &entity.0[1..];
            let mut address = 0u16;
            for c in address_str.as_bytes() {
                address *= 10;
                let next = c - '0' as u8;
                if next >= 10 {
                    return Err("invalid characters in entity");
                }
                address += next as u16;
            }
            if address == 0 {
                return Err("entity location cannot be 0");
            }
            Ok(address - 1) // address is 0-indexed
        }
    }
}
