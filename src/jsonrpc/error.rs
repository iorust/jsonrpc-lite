use std::error;
use std::fmt::{Formatter, Display, Result};
use std::string::ToString;
use serde_json::{Value, to_string};

/// Error Code
#[derive(Clone, PartialEq, Debug)]
enum ErrorCode {
    /// Invalid JSON was received by the server.
    /// An error occurred on the server while parsing the JSON text.
    ParseError,
    /// The JSON sent is not a valid Request object.
    InvalidRequest,
    /// The method does not exist / is not available.
    MethodNotFound,
    /// Invalid method parameter(s).
    InvalidParams,
    /// Internal JSON-RPC error.
    InternalError,
    /// Reserved for implementation-defined server-errors.
    ServerError(i64),
}

use self::ErrorCode::*;

impl ErrorCode {
    fn code(&self) -> i64 {
        match *self {
            ParseError => -32700,
            InvalidRequest => -32600,
            MethodNotFound => -32601,
            InvalidParams => -32602,
            InternalError => -32603,
            ServerError(code) => code,
        }
    }

    fn as_str(&self) -> &'static str {
        match *self {
            ParseError => "Parse error",
            InvalidRequest => "Invalid request",
            MethodNotFound => "Method not found",
            InvalidParams => "Invalid params",
            InternalError => "Internal error",
            ServerError(_) => "Server error",
        }
    }
}

impl ToString for ErrorCode {
    fn to_string(&self) -> String {
        String::from(self.as_str())
    }
}

/// Error Object
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Error {
    code: i64,
    message: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}
impl Error {
    fn new(code: ErrorCode) -> Self {
        Self::custom(code.code(), code.to_string())
    }

    pub fn parse_error() -> Self {
        Self::new(ParseError)
    }

    pub fn invalid_request() -> Self {
        Self::new(InvalidRequest)
    }

    pub fn method_not_found() -> Self {
        Self::new(MethodNotFound)
    }

    pub fn invalid_params() -> Self {
        Self::new(InvalidParams)
    }

    pub fn internal_error() -> Self {
        Self::new(InternalError)
    }

    pub fn server_error(code: i64) -> Self {
        Self::new(ServerError(code))
    }

    pub fn custom(code: i64, msg: String) -> Self {
        Error {
            code: code,
            message: msg,
            data: None,
        }
    }

    pub fn with_data<D: Into<Option<Value>>>(mut self, data: D) -> Self {
        self.data = data.into();
        self
    }

    pub fn code(&self) -> i64 {
        self.code
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn data(&self) -> Option<&Value> {
        self.data.as_ref()
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&to_string(&self).unwrap(), f)
    }
}
