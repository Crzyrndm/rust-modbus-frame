//! Exception codes as documented by https://en.wikipedia.org/wiki/Modbus#Exception_responses

pub use crate::Exception;
/// Function code received in the query is not recognized or allowed by slave
pub const ILLEGAL_FUNCTION: Exception = Exception(1);
/// Data address of some or all the required entities are not allowed or do not exist in slave
pub const ILLEGAL_ADDRESS: Exception = Exception(2); //
/// Value is not accepted by slave
pub const ILLEGAL_DATA: Exception = Exception(3);
/// Unrecoverable error occurred while slave was attempting to perform requested action
pub const DEVICE_FAILURE: Exception = Exception(4);
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
