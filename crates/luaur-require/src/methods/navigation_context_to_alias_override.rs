use crate::enums::navigate_result::NavigateResult;
use crate::records::navigation_context::NavigationContext;

impl NavigationContext {
    #[allow(unused_variables)]
    pub fn to_alias_override(&mut self, alias_unprefixed: &str) -> NavigateResult {
        NavigateResult::NotFound
    }
}
