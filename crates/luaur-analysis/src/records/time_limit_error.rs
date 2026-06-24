use crate::records::internal_compiler_error::InternalCompilerError;

#[derive(Debug, Clone)]
pub struct TimeLimitError {
    pub base: InternalCompilerError,
}

unsafe impl Send for TimeLimitError {}
unsafe impl Sync for TimeLimitError {}

#[cfg(feature = "std")]
impl std::error::Error for TimeLimitError {}

impl core::fmt::Display for TimeLimitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.base.message)
    }
}
