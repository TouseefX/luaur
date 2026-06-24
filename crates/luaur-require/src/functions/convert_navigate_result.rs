use crate::enums::navigate_result::NavigateResult;

#[allow(non_camel_case_types)]
pub type luarequire_NavigateResult = i32;

pub const NAVIGATE_SUCCESS: luarequire_NavigateResult = 0;
pub const NAVIGATE_AMBIGUOUS: luarequire_NavigateResult = 1;
pub const NAVIGATE_NOT_FOUND: luarequire_NavigateResult = 2;

pub(crate) fn convert_navigate_result(result: luarequire_NavigateResult) -> NavigateResult {
    if result == NAVIGATE_SUCCESS {
        return NavigateResult::Success;
    }
    if result == NAVIGATE_AMBIGUOUS {
        return NavigateResult::Ambiguous;
    }

    NavigateResult::NotFound
}
