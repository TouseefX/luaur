use crate::enums::navigate_result::NavigateResult;
use crate::records::navigator::Navigator;
use crate::type_aliases::error_require_navigator::Error;
use alloc::string::String;

impl Navigator {
    pub fn reset_to_requirer(&mut self) -> Error {
        let result = unsafe { &mut *self.navigation_context }.reset_to_requirer();
        if result == NavigateResult::Success {
            return None;
        }

        let mut error_message = String::from("could not reset to requiring context");
        if result == NavigateResult::Ambiguous {
            error_message.push_str(" (ambiguous)");
        }
        Some(error_message)
    }
}
