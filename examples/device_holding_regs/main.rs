use modbus::{builder, frame, rtu};
use modbus_frames as modbus;

const ADDRESS: u8 = 1;
struct DeviceState {
    device: modbus_frames::device::Device,
    holding_regs: [u16; 5],
}

fn main() {
    let mut state = DeviceState {
        device: modbus_frames::device::Device::new(ADDRESS),
        holding_regs: [0, 3, 6, 9, 12],
    };
    // combined rx/tx buffer
    // the receive_bytes function returns the unused portion which is then used to build the response
    // size requirement is the max of possible command + response sizes
    // for standard modbus, this is 264 bytes (256 is largest command/response, the paired response/command is max 8 bytes)
    let mut buffer = [0; 264];

    let (remainder, message) = receive_message(&mut buffer);

    let _response = if let Ok(frame) = message {
        // only respond to a valid frame
        if frame.device() == state.device {
            // only respond if frame is for this device
            let response = handle_frame(&mut state, frame, remainder);
            if let Some(response) = response {
                send_message(&response);
            }
        }
    };
}

fn receive_message<'a>(
    buffer: &'a mut [u8],
) -> (
    &'a mut [u8],
    Result<modbus::frame::Frame<'a>, modbus::error::Error>,
) {
    // receive a command for this device
    // function: read holding registers
    // starting address: 1
    // number of registers: 3
    // CRC is valid
    let bytes_received = [1, 3, 0, 1, 0, 3, 84, 11];
    buffer[..bytes_received.len()].copy_from_slice(&bytes_received);

    // try_from will check:
    // - that there is at least 4 bytes received (the minimum size of a modbus frame)
    // - that the last two bytes form a valid CRC for the message
    let (received, remainder) = buffer.split_at_mut(bytes_received.len());
    dbg!(&received);
    (remainder, rtu::decode(received))
}

fn handle_frame<'a, 'b>(
    state: &mut DeviceState,
    frame: frame::Frame<'a>,
    response_buffer: &'b mut [u8],
) -> Option<frame::Frame<'b>> {
    let response = match frame.function() {
        modbus::function::READ_HOLDING_REGISTERS => {
            handle_read_holding_register(state, frame, response_buffer)
        }
        _ => Some(
            builder::build_frame(response_buffer)
                .for_device(&state.device)
                .exception(frame.function(), modbus::exception::ILLEGAL_FUNCTION),
        ),
    };
    response
}

fn handle_read_holding_register<'a, 'f>(
    state: &DeviceState,
    read_frame: frame::Frame<'f>,
    response_buffer: &'a mut [u8],
) -> Option<frame::Frame<'a>> {
    let response_builder = builder::build_frame(response_buffer).for_device(&state.device);
    let holding_regs_req =
        modbus::transaction::read_holding_registers::Request::parse_from(&read_frame);
    if let Err(exception) = holding_regs_req {
        return exception
            .map(|ex| response_builder.exception(modbus::function::READ_HOLDING_REGISTERS, ex));
    }
    let request = holding_regs_req.unwrap();

    let start = request.address() as usize;
    let end = start + request.register_count() as usize;
    let exception = if start > state.holding_regs.len() {
        Some(modbus::exception::ILLEGAL_ADDRESS)
    } else if end > state.holding_regs.len() {
        Some(modbus::exception::ILLEGAL_DATA)
    } else {
        None
    };

    let x = match exception {
        Some(ex) => response_builder.exception(modbus::function::READ_HOLDING_REGISTERS, ex),
        None => response_builder
            .function(modbus::function::READ_HOLDING_REGISTERS)
            .registers(&state.holding_regs[start..end])
            .finalise(),
    };
    Some(x)
}

fn send_message<'a>(frame: &frame::Frame<'a>) {
    let transmitted = frame.raw_bytes();
    dbg!(transmitted);
    // send bytes...
}
