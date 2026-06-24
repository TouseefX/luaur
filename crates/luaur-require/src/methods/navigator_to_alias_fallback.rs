use crate::enums::navigate_result::NavigateResult;
use crate::records::navigation_context::NavigationContext;
use crate::records::navigator::Navigator;
use crate::type_aliases::error_require_navigator::Error;
use alloc::string::String;

impl Navigator {
    pub fn to_alias_fallback(&mut self, alias_unprefixed: &str) -> Error {
        let result = unsafe { (*self.navigation_context).to_alias_fallback(alias_unprefixed) };

        if result == NavigateResult::Success {
            return None;
        }

        let mut error_message = String::from("@");
        error_message.push_str(alias_unprefixed);
        error_message.push_str(" is not a valid alias");

        if result == NavigateResult::Ambiguous {
            error_message.push_str(" (ambiguous)");
        }

        Some(error_message)
    }
}
