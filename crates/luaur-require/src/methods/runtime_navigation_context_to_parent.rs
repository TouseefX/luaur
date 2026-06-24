use crate::enums::navigate_result::NavigateResult;
use crate::functions::convert_navigate_result::convert_navigate_result;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
    pub fn to_parent(&mut self) -> NavigateResult {
        if self.config.is_null() {
            return NavigateResult::NotFound;
        }
        let result =
            unsafe { (self.config.as_ref().unwrap().to_parent.unwrap())(self.l, self.ctx) };
        convert_navigate_result(result as i32)
    }
}
