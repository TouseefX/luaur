//! Error type and `Result` alias, mirroring mlua's [`Error`] / [`Result`].
//!
//! We expose the common, developer-facing subset of mlua's `Error` variants
//! (`RuntimeError`, `SyntaxError`, the two conversion errors, etc.) plus the
//! `Error::external` / `Error::runtime` constructors. Variants specific to
//! features we have not implemented (async, serde, scopes, registry keys) are
//! intentionally omitted.

use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

/// A boxed standard error, used by [`Error::ExternalError`].
type DynStdError = dyn StdError + Send + Sync;

/// The result type used throughout `luaur-rt`, mirroring [`mlua::Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when interacting with the Lua engine.
///
/// The variant set mirrors the commonly used part of mlua's `Error`. It is
/// marked `#[non_exhaustive]` (like mlua's) so new variants can be added
/// without a breaking change.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    /// A Lua syntax (compile/parse) error.
    SyntaxError {
        /// The human-readable message produced by the compiler.
        message: String,
        /// Whether the input looked like it was merely incomplete (e.g. an
        /// unterminated block). Always `false` for now; reserved for REPL use.
        incomplete_input: bool,
    },
    /// A Lua runtime error (`error(..)`, a failed `assert`, a type error, or a
    /// Rust callback returning `Err`).
    RuntimeError(String),
    /// A memory allocation error reported by the VM.
    MemoryError(String),
    /// A value could not be converted **from** a Lua value into the requested
    /// Rust type.
    FromLuaConversionError {
        /// The Lua type name of the source value.
        from: &'static str,
        /// The name of the target Rust type.
        to: String,
        /// Optional extra detail.
        message: Option<String>,
    },
    /// A Rust value could not be converted **into** a Lua value.
    ToLuaConversionError {
        /// The name of the source Rust type.
        from: &'static str,
        /// The Lua type name being targeted.
        to: &'static str,
        /// Optional extra detail.
        message: Option<String>,
    },
    /// A `UserData` value was accessed as the wrong concrete type.
    UserDataTypeMismatch,
    /// A `UserData` value was used after it had been destructed (dropped).
    UserDataDestructed,
    /// A `UserData` could not be mutably borrowed because it is already
    /// borrowed.
    UserDataBorrowMutError,
    /// An error originating outside Lua, wrapped via [`Error::external`].
    ExternalError(Arc<DynStdError>),
}

impl Error {
    /// Create a [`Error::RuntimeError`] from any displayable message.
    ///
    /// Mirrors `mlua::Error::runtime`.
    pub fn runtime<S: fmt::Display>(message: S) -> Self {
        Error::RuntimeError(message.to_string())
    }

    /// Wrap an arbitrary `std::error::Error` as an [`Error::ExternalError`].
    ///
    /// Mirrors `mlua::Error::external`.
    pub fn external<T: Into<Box<DynStdError>>>(err: T) -> Self {
        Error::ExternalError(err.into().into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SyntaxError { message, .. } => write!(f, "syntax error: {message}"),
            Error::RuntimeError(msg) => write!(f, "runtime error: {msg}"),
            Error::MemoryError(msg) => write!(f, "memory error: {msg}"),
            Error::FromLuaConversionError { from, to, message } => {
                write!(f, "error converting Lua {from} to {to}")?;
                if let Some(m) = message {
                    write!(f, " ({m})")?;
                }
                Ok(())
            }
            Error::ToLuaConversionError { from, to, message } => {
                write!(f, "error converting {from} to Lua {to}")?;
                if let Some(m) = message {
                    write!(f, " ({m})")?;
                }
                Ok(())
            }
            Error::UserDataTypeMismatch => write!(f, "userdata type mismatch"),
            Error::UserDataDestructed => write!(f, "userdata used after being destructed"),
            Error::UserDataBorrowMutError => write!(f, "userdata already mutably borrowed"),
            Error::ExternalError(err) => write!(f, "{err}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::ExternalError(err) => Some(&**err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::external(err)
    }
}
