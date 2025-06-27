//! JSON-RPC 2.0 errors
//!
//! This module provides error handling types and utilities for JSON-RPC 2.0 protocol.
//! It includes standard error codes as defined in the JSON-RPC 2.0 specification
//! and a comprehensive Error type for representing JSON-RPC errors.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error;
use std::fmt;
use std::result;

/// Standard JSON-RPC 2.0 error codes
///
/// This enum represents the predefined error codes as specified in the JSON-RPC 2.0
/// specification, along with support for custom server error codes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    /// Invalid JSON was received by the server.
    /// An error occurred on the server while parsing the JSON text.
    /// Error code: -32700
    ParseError,
    /// The JSON sent is not a valid Request object.
    /// Error code: -32600
    InvalidRequest,
    /// The method does not exist / is not available.
    /// Error code: -32601
    MethodNotFound,
    /// Invalid method parameter(s).
    /// Error code: -32602
    InvalidParams,
    /// Internal JSON-RPC error.
    /// Error code: -32603
    InternalError,
    /// Reserved for implementation-defined server-errors.
    /// Error codes from -32000 to -32099 are reserved for implementation-defined server-errors.
    /// The `i64` value represents the custom error code.
    ServerError(i64),
}

impl ErrorCode {
    /// Returns the numeric error code as defined in the JSON-RPC 2.0 specification
    ///
    /// # Returns
    ///
    /// The corresponding error code number:
    /// - ParseError: -32700
    /// - InvalidRequest: -32600
    /// - MethodNotFound: -32601
    /// - InvalidParams: -32602
    /// - InternalError: -32603
    /// - ServerError: the custom code provided
    pub fn code(&self) -> i64 {
        match *self {
            ErrorCode::ParseError => -32700,
            ErrorCode::InvalidRequest => -32600,
            ErrorCode::MethodNotFound => -32601,
            ErrorCode::InvalidParams => -32602,
            ErrorCode::InternalError => -32603,
            ErrorCode::ServerError(code) => code,
        }
    }

    /// Returns a human-readable description of the error
    ///
    /// # Returns
    ///
    /// A static string slice containing the error description
    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorCode::ParseError => "Parse error",
            ErrorCode::InvalidRequest => "Invalid request",
            ErrorCode::MethodNotFound => "Method not found",
            ErrorCode::InvalidParams => "Invalid params",
            ErrorCode::InternalError => "Internal error",
            ErrorCode::ServerError(_) => "Server error",
        }
    }
}

impl fmt::Display for ErrorCode {
    /// Formats the error code as a string
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the error code to
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// JSON-RPC 2.0 Error Object
///
/// Represents an error that occurred during the processing of a JSON-RPC request.
/// This struct follows the JSON-RPC 2.0 specification for error objects.
///
/// # Fields
///
/// * `code` - A number that indicates the error type that occurred
/// * `message` - A string providing a short description of the error
/// * `data` - Optional additional information about the error
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Error {
    /// The error code number
    pub code: i64,
    /// A short description of the error
    pub message: String,
    /// Optional additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Error {
    /// Creates a new Error from an ErrorCode
    ///
    /// # Arguments
    ///
    /// * `code` - The error code to create the error from
    ///
    /// # Returns
    ///
    /// A new `Error` instance with the appropriate code and message
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonrpc_lite::error::{Error, ErrorCode};
    ///
    /// let error = Error::new(ErrorCode::MethodNotFound);
    /// assert_eq!(error.code, -32601);
    /// assert_eq!(error.message, "Method not found");
    /// ```
    pub fn new(code: ErrorCode) -> Self {
        Error {
            code: code.code(),
            message: code.to_string(),
            data: None,
        }
    }

    /// Creates a parse error (-32700)
    ///
    /// # Returns
    ///
    /// A new `Error` instance representing a parse error
    pub fn parse_error() -> Self {
        Self::new(ErrorCode::ParseError)
    }

    /// Creates an invalid request error (-32600)
    ///
    /// # Returns
    ///
    /// A new `Error` instance representing an invalid request error
    pub fn invalid_request() -> Self {
        Self::new(ErrorCode::InvalidRequest)
    }

    /// Creates a method not found error (-32601)
    ///
    /// # Returns
    ///
    /// A new `Error` instance representing a method not found error
    pub fn method_not_found() -> Self {
        Self::new(ErrorCode::MethodNotFound)
    }

    /// Creates an invalid params error (-32602)
    ///
    /// # Returns
    ///
    /// A new `Error` instance representing an invalid params error
    pub fn invalid_params() -> Self {
        Self::new(ErrorCode::InvalidParams)
    }

    /// Creates an internal error (-32603)
    ///
    /// # Returns
    ///
    /// A new `Error` instance representing an internal error
    pub fn internal_error() -> Self {
        Self::new(ErrorCode::InternalError)
    }
}

impl error::Error for Error {
    /// Returns a description of the error
    ///
    /// # Returns
    ///
    /// A string slice containing the error message
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    /// Formats the error as a JSON string for display
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).unwrap_or_else(|_| self.message.clone())
        )
    }
}

/// A type alias for `Result<T, Error>`
///
/// This is a convenience type that represents either a successful result of type `T`
/// or a JSON-RPC error.
pub type Result<T> = result::Result<T, Error>;
