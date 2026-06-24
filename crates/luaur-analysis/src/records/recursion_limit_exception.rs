use crate::records::internal_compiler_error::InternalCompilerError;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct RecursionLimitException {
    pub base: InternalCompilerError,
}

unsafe impl Send for RecursionLimitException {}
unsafe impl Sync for RecursionLimitException {}

#[cfg(feature = "std")]
impl std::error::Error for RecursionLimitException {}

impl core::fmt::Display for RecursionLimitException {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.base.message)
    }
}
