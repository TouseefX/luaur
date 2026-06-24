use crate::records::file_navigation_context::FileNavigationContext;
use luaur_cli_lib::functions::is_file::is_file;

#[allow(non_snake_case)]
pub unsafe fn file_navigation_context_is_module_present(
    this: *const FileNavigationContext,
) -> bool {
    let this = &*this;
    is_file(&this.vfs.get_absolute_file_path())
}
