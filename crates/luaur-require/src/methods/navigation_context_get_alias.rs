use crate::enums::navigate_result::NavigateResult;
use crate::records::navigation_context::NavigationContext;

impl NavigationContext {
    pub fn get_alias(&self, _alias: &str) -> Option<alloc::string::String> {
        None
    }
}
