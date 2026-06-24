use crate::records::error_handler::ErrorHandler;
use crate::records::navigation_context::NavigationContextTrait;

#[allow(non_camel_case_types)]
pub struct Navigator {
    pub(crate) navigation_context: *mut dyn NavigationContextTrait,
    pub(crate) error_handler: *mut dyn ErrorHandler,
}

impl Navigator {
    pub const Status: () = ();
}
