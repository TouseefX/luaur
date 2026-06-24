use crate::enums::navigate_result::NavigateResult;
use crate::functions::convert_navigate_result::convert_navigate_result;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
    pub fn reset_to_requirer(&mut self) -> NavigateResult {
        unsafe {
            let config_ptr = self.config;
            if config_ptr.is_null() {
                return NavigateResult::NotFound;
            }
            let config = &*config_ptr;
            if config.reset.is_none() {
                return NavigateResult::NotFound;
            }

            let result = (config.reset.unwrap())(
                self.l,
                self.ctx,
                self.requirer_chunkname.as_ptr() as *const core::ffi::c_char,
            );

            convert_navigate_result(result as i32)
        }
    }
}
