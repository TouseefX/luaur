use crate::records::file_navigation_context::FileNavigationContext;
use alloc::string::String;
use core::option::Option;

#[allow(non_snake_case)]
pub unsafe fn file_navigation_context_get_alias(
    _this: *const FileNavigationContext,
    _alias: &String,
) -> Option<String> {
    None
}
