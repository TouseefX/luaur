use crate::enums::navigate_result::NavigateResult;
use crate::functions::convert_navigate_result::convert_navigate_result;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
    pub fn to_alias_override(&mut self, alias_unprefixed: &str) -> NavigateResult {
        if self.config.is_null() {
            return NavigateResult::NotFound;
        }
        let config = unsafe { &*self.config };
        if config.to_alias_override.is_none() {
            return NavigateResult::NotFound;
        }
        let result = unsafe {
            (config.to_alias_override.unwrap())(
                self.l,
                self.ctx,
                alias_unprefixed.as_ptr() as *const core::ffi::c_char,
            )
        };
        convert_navigate_result(result as i32)
    }
}
