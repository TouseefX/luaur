extern crate alloc;

use alloc::string::String;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct CompileError {
    pub(crate) location: Location,
    pub(crate) message: String,
}

impl core::fmt::Display for CompileError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CompileError {}
