//! misc timing requirements

use core::time::Duration;
// all timings specified here assume use of 11 bits per char (8N2 / 8E1 / 8O1 character formats)

/// each message should be followed by atleast 3.5 characters of silence to signal completion
/// this signals the bus is idle and all devices should prep to receive again
pub fn inter_frame_delay_unclamped(baud: u32) -> Duration {
    const BREAK_FACTOR: Duration = Duration::from_micros((3.5 * 11.0 * 1_000_000.0) as u64);
    BREAK_FACTOR / baud
}

/// each message should be followed by atleast 3.5 characters of silence to signal completion
/// this signals the bus is idle and all devices should prep to receive again
/// Additionally, according to the Modbus RTU standard, the minimum silent period should
/// be 1.75 ms regardless of the baud rate.
/// see https://modbus.org/docs/Modbus_over_serial_line_V1_02.pdf pg.13
pub fn inter_frame_delay(baud: u32) -> Duration {
    inter_frame_delay_unclamped(baud).max(Duration::from_micros(1_500))
}

/// if no characters are received for >1.5 charaters, a message is assumed to have ended
/// after this point the device should complete processing and assume the next byte is the start of a new frame
pub fn inter_character_time_unclamped(baud: u32) -> Duration {
    const BREAK_FACTOR: Duration = Duration::from_micros((1.5 * 11.0 * 1_000_000.0) as u64);
    BREAK_FACTOR / baud
}

/// if no characters are received for >1.5 charaters, a message is assumed to have ended
/// after this point the device should complete processing and assume the next byte is the start of a new frame
/// Additionally, according to the modbus standard, the minimum inter characetr time should be clamped to 750us
/// see https://modbus.org/docs/Modbus_over_serial_line_V1_02.pdf pg 13
pub fn inter_character_time(baud: u32) -> Duration {
    const BREAK_FACTOR: Duration = Duration::from_micros((1.5 * 11.0 * 1_000_000.0) as u64);
    BREAK_FACTOR / baud
}
