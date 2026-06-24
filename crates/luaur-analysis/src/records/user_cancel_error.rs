use crate::records::internal_compiler_error::InternalCompilerError;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct UserCancelError {
    pub base: InternalCompilerError,
}

unsafe impl Send for UserCancelError {}
unsafe impl Sync for UserCancelError {}

#[cfg(feature = "std")]
impl std::error::Error for UserCancelError {}

impl core::fmt::Display for UserCancelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.base.message)
    }
}
