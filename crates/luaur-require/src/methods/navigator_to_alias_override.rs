use crate::enums::navigate_result::NavigateResult;
use crate::records::navigator::Navigator;
use crate::type_aliases::error_require_navigator::Error;

impl Navigator {
    pub fn to_alias_override(&mut self, alias_unprefixed: &str) -> (Error, bool) {
        match unsafe { &mut *self.navigation_context }.to_alias_override(alias_unprefixed) {
            NavigateResult::Success => (None, true),
            NavigateResult::NotFound => (None, false),
            NavigateResult::Ambiguous => (
                Some(format!(
                    "@{} is not a valid alias (ambiguous)",
                    alias_unprefixed
                )),
                false,
            ),
        }
    }
}
