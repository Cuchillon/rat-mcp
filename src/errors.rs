use std::borrow::Cow;
use std::fmt::Display;

use rmcp::ErrorData as McpError;

pub(crate) fn internal_error(error: impl Display) -> McpError {
    McpError::internal_error(error.to_string(), None)
}

pub(crate) fn internal_error_with_message(message: impl Into<Cow<'static, str>>) -> McpError {
    McpError::internal_error(message, None)
}

pub(crate) fn invalid_params(message: impl Into<Cow<'static, str>>) -> McpError {
    McpError::invalid_params(message, None)
}
