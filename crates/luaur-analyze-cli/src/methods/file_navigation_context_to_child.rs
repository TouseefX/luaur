use crate::functions::convert_analyze_requirer::convert;
use crate::records::file_navigation_context::FileNavigationContext;
use alloc::string::String;

#[allow(non_snake_case)]
pub unsafe fn file_navigation_context_to_child(
    this: *mut FileNavigationContext,
    component: &String,
) -> crate::type_aliases::navigate_result::NavigateResult {
    let this = &mut *this;
    convert(this.vfs.to_child(component.as_str()))
}
