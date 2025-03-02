//! Error handling for GraphViz operations.
//!
//! This module defines the primary error type `GraphvizError` which encapsulates
//! all potential error conditions that may arise during GraphViz operations.

use std::error::Error;
use std::fmt;
use std::ffi::NulError;

/// Enumeration of all possible errors that can occur during GraphViz operations.
#[derive(Debug)]
pub enum GraphvizError {
    /// Error creating a graph structure
    GraphCreationFailed,
    /// Error creating a node structure
    NodeCreationFailed,
    /// Error creating an edge structure
    EdgeCreationFailed,
    /// Error during layout computation
    LayoutFailed,
    /// Error during rendering process
    RenderFailed,
    /// Invalid string for C FFI (contains null bytes)
    InvalidString,
    /// Error setting an attribute
    AttributeSetFailed,
    /// Error getting an attribute
    AttributeGetFailed,
    /// Error freeing layout resources
    FreeLayoutFailed,
    /// String is not valid UTF-8
    InvalidUtf8,
    /// Null pointer encountered
    NullPointer(&'static str),
    /// Context creation failed
    ContextCreationFailed,
    /// Invalid format specified
    InvalidFormat,
    /// Invalid engine specified
    InvalidEngine,
    /// Failed to initialize GraphViz
    InitializationFailed,
    /// Failed to clean up GraphViz resources
    CleanupFailed,
    /// System error (with errno)
    SystemError(i32),
    /// File I/O error
    IoError(std::io::Error),
}

impl fmt::Display for GraphvizError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphvizError::GraphCreationFailed => write!(f, "Failed to create graph"),
            GraphvizError::NodeCreationFailed => write!(f, "Failed to create node"),
            GraphvizError::EdgeCreationFailed => write!(f, "Failed to create edge"),
            GraphvizError::LayoutFailed => write!(f, "Failed to compute layout"),
            GraphvizError::RenderFailed => write!(f, "Failed to render graph"),
            GraphvizError::InvalidString => write!(f, "String contains null bytes"),
            GraphvizError::AttributeSetFailed => write!(f, "Failed to set attribute"),
            GraphvizError::AttributeGetFailed => write!(f, "Failed to get attribute"),
            GraphvizError::FreeLayoutFailed => write!(f, "Failed to free layout resources"),
            GraphvizError::InvalidUtf8 => write!(f, "String is not valid UTF-8"),
            GraphvizError::NullPointer(context) => write!(f, "Null pointer encountered: {}", context),
            GraphvizError::ContextCreationFailed => write!(f, "Failed to create GraphViz context"),
            GraphvizError::InvalidFormat => write!(f, "Invalid output format specified"),
            GraphvizError::InvalidEngine => write!(f, "Invalid layout engine specified"),
            GraphvizError::InitializationFailed => write!(f, "Failed to initialize GraphViz"),
            GraphvizError::CleanupFailed => write!(f, "Failed to clean up GraphViz resources"),
            GraphvizError::SystemError(errno) => write!(f, "System error occurred (errno: {})", errno),
            GraphvizError::IoError(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl Error for GraphvizError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GraphvizError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<NulError> for GraphvizError {
    fn from(_: NulError) -> Self {
        GraphvizError::InvalidString
    }
}

impl From<std::io::Error> for GraphvizError {
    fn from(err: std::io::Error) -> Self {
        GraphvizError::IoError(err)
    }
}

impl From<std::str::Utf8Error> for GraphvizError {
    fn from(_: std::str::Utf8Error) -> Self {
        GraphvizError::InvalidUtf8
    }
}
