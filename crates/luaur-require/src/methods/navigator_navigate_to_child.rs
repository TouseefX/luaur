use crate::enums::navigate_result::NavigateResult;
use crate::records::navigator::Navigator;
use crate::type_aliases::error_require_navigator::Error;
use alloc::string::String;

impl Navigator {
    pub fn navigate_to_child(&mut self, component: &str) -> Error {
        let result = unsafe { &mut *self.navigation_context }.to_child(component);
        if result == NavigateResult::Success {
            return None;
        }

        let mut error_message = String::from("could not resolve child component \"");
        error_message.push_str(component);
        error_message.push('"');
        if result == NavigateResult::Ambiguous {
            error_message.push_str(" (ambiguous)");
        }
        Some(error_message)
    }
}
