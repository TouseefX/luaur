use crate::functions::convert_analyze_requirer::convert;
use crate::records::file_navigation_context::FileNavigationContext;
use crate::type_aliases::navigate_result::NavigateResult;

#[allow(non_snake_case)]
pub unsafe fn file_navigation_context_to_parent(
    this: *mut FileNavigationContext,
) -> NavigateResult {
    let this = &mut *this;
    let status = this.vfs.to_parent();
    convert(status)
}
