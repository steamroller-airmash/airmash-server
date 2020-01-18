use airmash_protocol_v5::{DeserializeError, SerializeError};
use tokio_tungstenite::tungstenite::Error as WsError;

use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug)]
pub enum ClientError {
    WebSocket(WsError),
    Serialize(SerializeError),
    Deserialize(DeserializeError),
    InvalidWsFrame(String),
    Custom(Box<dyn Error + Send + 'static>),
}

impl From<WsError> for ClientError {
    fn from(e: WsError) -> Self {
        ClientError::WebSocket(e)
    }
}

impl From<SerializeError> for ClientError {
    fn from(e: SerializeError) -> Self {
        ClientError::Serialize(e)
    }
}

impl From<DeserializeError> for ClientError {
    fn from(e: DeserializeError) -> Self {
        ClientError::Deserialize(e)
    }
}

impl Display for ClientError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        use self::ClientError::*;
        match self {
            WebSocket(e) => write!(fmt, "WebSocket({})", e),
            Serialize(e) => write!(fmt, "Serialize({})", e),
            Deserialize(e) => write!(fmt, "Deserialize({})", e),
            InvalidWsFrame(desc) => write!(fmt, "InvalidWsFrame({})", desc),
            Custom(e) => write!(fmt, "Custom({})", e),
        }
    }
}

impl Error for ClientError {}
