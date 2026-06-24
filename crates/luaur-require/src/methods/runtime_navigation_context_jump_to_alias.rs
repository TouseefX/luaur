use crate::enums::navigate_result::NavigateResult;
use crate::functions::convert_navigate_result::convert_navigate_result;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
    pub fn jump_to_alias(&mut self, path: &str) -> NavigateResult {
        unsafe {
            let config_ptr = self.config;
            if config_ptr.is_null() {
                return NavigateResult::NotFound;
            }

            let config = &*config_ptr;
            if config.jump_to_alias.is_none() {
                return NavigateResult::NotFound;
            }

            let result = (config.jump_to_alias.unwrap())(
                self.l,
                self.ctx,
                path.as_ptr() as *const core::ffi::c_char,
            );

            convert_navigate_result(result as i32)
        }
    }
}
