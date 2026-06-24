pub trait ErrorHandler {
    fn report_error(&mut self, message: alloc::string::String);
}

impl core::fmt::Debug for dyn ErrorHandler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ErrorHandler").finish_non_exhaustive()
    }
}
