use crate::{frame::Frame, Exception};

/// The data inside the modbus frame, normally identified by the function code
pub trait ModbusPayload<T> {
    /// The request type associated with this transaction.
    /// This will either be 'T' or the type which will trigger 'T' as the response
    type Request;
    /// The response type associated with this transaction.
    /// This will either be 'T' or the type which will be sent in response to this request
    type Response;

    /// parse frame payload into a usable form
    /// if parsing fails
    /// in the case where the payload is an exception response, that will be the error condition: Err(<exception code>)
    /// in the case where we are parsing a request
    /// - a Err(None) response indicates that no response should be generated
    /// - a Err(<code>) response that an exception should be returned
    fn parse_payload<'b>(frame: &Frame<'b>) -> Result<T, Option<Exception>>;
}
