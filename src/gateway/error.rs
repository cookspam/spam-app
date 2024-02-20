use solana_client_wasm::ClientError;

pub type GatewayResult<T> = Result<T, GatewayError>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GatewayError {
    DeserializationFailure,
    NotFound,
    NetworkUnavailable,
    Unknown,
}

impl From<ClientError> for GatewayError {
    fn from(err: ClientError) -> Self {
        let msg = err.to_string();
        if msg.starts_with("Client error: Invalid param: could not find account")
            || msg.starts_with("Client error: AccountNotFound: ")
        {
            GatewayError::NotFound
        } else if msg.starts_with("Client error: error sending request") {
            GatewayError::NetworkUnavailable
        } else {
            GatewayError::Unknown
        }
    }
}