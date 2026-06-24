use crate::functions::convert_analyze_requirer::convert;
use crate::records::file_navigation_context::FileNavigationContext;
use crate::type_aliases::navigate_result::NavigateResult;
use alloc::string::String;
use luaur_cli_lib::functions::is_absolute_path::is_absolute_path;

#[allow(non_snake_case)]
pub unsafe fn file_navigation_context_jump_to_alias(
    this: *mut FileNavigationContext,
    path: &String,
) -> NavigateResult {
    let this = &mut *this;

    if !is_absolute_path(path.as_str()) {
        return NavigateResult::NotFound;
    }

    let status = this.vfs.reset_to_path(path.as_str());
    convert(status)
}
