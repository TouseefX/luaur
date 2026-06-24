use crate::records::file_navigation_context::FileNavigationContext;
use luaur_cli_lib::records::vfs_navigator::VfsNavigator;

impl FileNavigationContext {
    /// `FileNavigationContext::FileNavigationContext(std::string requirerPath)`
    /// (`CLI/src/AnalyzeRequirer.cpp:31-34`): `requirerPath(std::move(requirerPath))`.
    pub fn new(requirer_path: alloc::string::String) -> Self {
        FileNavigationContext {
            requirer_path,
            vfs: VfsNavigator::default(),
            interrupt_info: None,
        }
    }
}

/// Compatibility shim for the pinned skeleton name: assigns `requirerPath` onto an
/// existing context (the in-place member-init form).
#[allow(non_snake_case)]
pub unsafe fn file_navigation_context_file_navigation_context(
    this: &mut FileNavigationContext,
    requirer_path: alloc::string::String,
) {
    this.requirer_path = requirer_path;
}
