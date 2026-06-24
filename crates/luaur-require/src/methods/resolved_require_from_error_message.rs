use crate::enums::status_require_impl::Status;
use crate::records::resolved_require::ResolvedRequire;

impl ResolvedRequire {
    pub fn from_error_message(message: &str) -> ResolvedRequire {
        ResolvedRequire {
            status: Status::ErrorReported,
            chunkname: alloc::string::String::new(),
            loadname: alloc::string::String::new(),
            cacheKey: alloc::string::String::new(),
            error: alloc::string::String::from(message),
        }
    }
}
