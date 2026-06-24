use crate::type_aliases::navigate_result::NavigateResult;
use luaur_cli_lib::enums::navigation_status::NavigationStatus;

pub fn convert(status: NavigationStatus) -> NavigateResult {
    match status {
        NavigationStatus::Success => NavigateResult::Success,
        NavigationStatus::Ambiguous => NavigateResult::Ambiguous,
        NavigationStatus::NotFound => NavigateResult::NotFound,
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use convert as convert_navigation_status;
