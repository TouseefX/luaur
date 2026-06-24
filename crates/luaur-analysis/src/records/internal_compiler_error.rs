use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct InternalCompilerError {
    pub message: alloc::string::String,
    pub module_name: Option<alloc::string::String>,
    pub location: Option<Location>,
}

unsafe impl Send for InternalCompilerError {}
unsafe impl Sync for InternalCompilerError {}

#[cfg(feature = "std")]
impl std::error::Error for InternalCompilerError {}

impl core::fmt::Display for InternalCompilerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}
