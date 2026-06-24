use crate::records::error_handler::ErrorHandler;
use crate::records::navigation_context::NavigationContextTrait;
use crate::records::navigator::Navigator;

impl Navigator {
    #[allow(unsafe_code)]
    pub fn new(
        navigation_context: &mut dyn NavigationContextTrait,
        error_handler: &mut dyn ErrorHandler,
    ) -> Self {
        let navigation_context_ptr = unsafe {
            core::mem::transmute::<
                &mut dyn NavigationContextTrait,
                *mut (dyn NavigationContextTrait + 'static),
            >(navigation_context)
        };
        let error_handler_ptr = unsafe {
            core::mem::transmute::<&mut dyn ErrorHandler, *mut (dyn ErrorHandler + 'static)>(
                error_handler,
            )
        };
        Self {
            navigation_context: navigation_context_ptr,
            error_handler: error_handler_ptr,
        }
    }
}

#[allow(unsafe_code)]
unsafe impl Send for Navigator {}
#[allow(unsafe_code)]
unsafe impl Sync for Navigator {}
