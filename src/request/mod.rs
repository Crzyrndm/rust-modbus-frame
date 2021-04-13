/// 0x01
pub struct ReadCoils {
    address: u16,
    quantity: u16,
}

/// 0x02
pub struct ReadDiscreteInputs {
    address: u16,
    quantity: u16,
}

/// 0x03
pub struct ReadHoldingRegisters {
    address: u16,
    quantity: u16,
}

/// 0x04
pub struct ReadInputRegisters {
    address: u16,
    quantity: u16,
}

/// 0x05
/// Success response is an echo of the command
pub struct WriteCoil {
    address: u16,
    /// valid values are 0xFF00 (ON) and 0x00 (OFF). Any other value is invalid and should be rejected (Illegal data)
    new_state: u16,
}

/// 0x06
/// Success response is an echo of the command
pub struct WriteRegister {
    address: u16,
    new_value: u16,
}

/// 0x0F
/// success response echos the address/quantity
pub struct WriteMultipleCoils<'b> {
    address: u16,
    quantity: u16,
    /// 1 = ON, 0 = OFF
    /// bytes are ordered from high to low address
    /// MSB of each byte has the highest address, extend with 0 bits
    bitmap: &'b [u8],
}

/// 0x10
/// success response echos the address/quantity
pub struct WriteMultipleRegisters<'b> {
    address: u16,
    quantity: u16,
    /// NOTE: registers will be in big endian format so won't make any sense to read on many systems
    /// convert to native before use
    registers: &'b [u16],
}

pub struct ReportId {
    // no data
}

pub struct FileSubRequest {
    reference_type: u8,
    file_number: u16,
    record_number: u16,
    record_length: u16,
}

/// 0x14
pub struct ReadFileRecord<'b> {
    subrequests: &'b [FileSubRequest],
}

/// 0x15
/// response echos request
pub struct WriteFileRecord<'a, 'b> {
    subrequests: &'b [(FileSubRequest, &'a [u16])],
}

/// 0x16
/// Result = (Current Contents AND And_Mask) OR (Or_Mask AND (NOT And_Mask))
pub struct MaskWriteRegister {
    address: u16,
    and_mask: u16,
    or_mask: u16,
}

/// 0x17
/// read and write in a single transaction
/// write operation is completed before read begins
/// response is standard read registers
pub struct ReadWriteMultipleRegisters<'b> {
    read_address: u16,
    read_quantity: u16,
    write_address: u16,
    write_quantity: u16,
    write_values: &'b [u16],
}

/// 0x18
///
pub struct ReadFifoQueue {
    fifo_address: u16,
}

pub struct EncapsulatedInterfaceTransport<Data> {
    mei_type: u8,
    mei_data: Data,
}

/// 0x2B
/// mei_type = 0x0D (13d)?
pub struct CanOpenPdu<'b> {
    can_data: &'b [u8],
}

/// 0x2B
/// mei_type = 0x0E (14d)?
pub struct ReadDeviceId {
    // 1 = basic, 2 = regular, 3 = extended, 4 = device specific
    read_id_code: u8,
    // the data id
    object_id: u8,
}
