#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum luarequire_NavigateResult {
    NAVIGATE_SUCCESS,
    NAVIGATE_AMBIGUOUS,
    NAVIGATE_NOT_FOUND,
}

#[allow(non_upper_case_globals)]
pub const NAVIGATE_SUCCESS: luarequire_NavigateResult = luarequire_NavigateResult::NAVIGATE_SUCCESS;
#[allow(non_upper_case_globals)]
pub const NAVIGATE_AMBIGUOUS: luarequire_NavigateResult =
    luarequire_NavigateResult::NAVIGATE_AMBIGUOUS;
#[allow(non_upper_case_globals)]
pub const NAVIGATE_NOT_FOUND: luarequire_NavigateResult =
    luarequire_NavigateResult::NAVIGATE_NOT_FOUND;

pub type LuarequireNavigateResult = luarequire_NavigateResult;
