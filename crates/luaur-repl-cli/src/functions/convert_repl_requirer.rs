use luaur_cli_lib::enums::navigation_status::NavigationStatus;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum luarequire_NavigateResult {
    NAVIGATE_SUCCESS = 0,
    NAVIGATE_AMBIGUOUS = 1,
    NAVIGATE_NOT_FOUND = 2,
}

pub fn convert(status: NavigationStatus) -> luarequire_NavigateResult {
    if status == NavigationStatus::Success {
        luarequire_NavigateResult::NAVIGATE_SUCCESS
    } else if status == NavigationStatus::Ambiguous {
        luarequire_NavigateResult::NAVIGATE_AMBIGUOUS
    } else {
        luarequire_NavigateResult::NAVIGATE_NOT_FOUND
    }
}
