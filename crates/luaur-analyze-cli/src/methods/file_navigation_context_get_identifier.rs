use crate::records::file_navigation_context::FileNavigationContext;
use alloc::string::String;
use core::option::Option;

#[allow(non_snake_case)]
pub unsafe fn file_navigation_context_get_identifier(
    this: *const FileNavigationContext,
) -> Option<String> {
    let this = &*this;
    Some(this.vfs.get_absolute_file_path())
}
