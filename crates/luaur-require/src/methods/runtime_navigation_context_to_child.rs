use crate::enums::navigate_result::NavigateResult;
use crate::functions::convert_navigate_result::convert_navigate_result;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
    pub fn to_child(&self, component: &str) -> NavigateResult {
        unsafe {
            let config_ptr = self.config;
            if config_ptr.is_null() {
                return NavigateResult::NotFound;
            }
            let config = &*config_ptr;
            if config.to_child.is_none() {
                return NavigateResult::NotFound;
            }

            let result = (config.to_child.unwrap())(
                self.l,
                self.ctx,
                component.as_ptr() as *const core::ffi::c_char,
            );

            convert_navigate_result(result as i32)
        }
    }
}
