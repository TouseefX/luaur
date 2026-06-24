use crate::enums::navigate_result::NavigateResult;
use crate::functions::convert_navigate_result::convert_navigate_result;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
    pub fn to_alias_fallback(&mut self, alias_unprefixed: &str) -> NavigateResult {
        unsafe {
            let config_ptr = self.config;
            if config_ptr.is_null() {
                return NavigateResult::NotFound;
            }

            let config = &*config_ptr;
            if config.to_alias_fallback.is_none() {
                return NavigateResult::NotFound;
            }

            let result = (config.to_alias_fallback.unwrap())(
                self.l,
                self.ctx,
                alias_unprefixed.as_ptr() as *const core::ffi::c_char,
            );

            convert_navigate_result(result as i32)
        }
    }
}
