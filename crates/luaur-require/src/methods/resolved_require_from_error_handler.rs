use alloc::string::String;

use crate::enums::status_require_impl::Status;
use crate::records::resolved_require::ResolvedRequire;
use crate::records::runtime_error_handler::RuntimeErrorHandler;

impl ResolvedRequire {
    pub fn resolved_require_from_error_handler(
        error_handler: &RuntimeErrorHandler,
    ) -> ResolvedRequire {
        ResolvedRequire {
            status: Status::ErrorReported,
            chunkname: alloc::string::String::new(),
            loadname: alloc::string::String::new(),
            cacheKey: alloc::string::String::new(),
            error: alloc::string::String::from(error_handler.get_reported_error()),
        }
    }
}
