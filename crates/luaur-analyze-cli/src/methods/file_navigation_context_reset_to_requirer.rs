use crate::functions::convert_analyze_requirer::convert;
use crate::records::file_navigation_context::FileNavigationContext;
use crate::type_aliases::navigate_result::NavigateResult;
use luaur_cli_lib::methods::vfs_navigator_reset_to_std_in::vfs_navigator_reset_to_std_in;

/// `NavigateResult FileNavigationContext::resetToRequirer()` (`CLI/src/AnalyzeRequirer.cpp:37-42`):
///
/// ```cpp
/// if (requirerPath == "-")
///     return convert(vfs.resetToStdIn());
/// return convert(vfs.resetToPath(requirerPath));
/// ```
#[allow(non_snake_case)]
pub unsafe fn file_navigation_context_reset_to_requirer(
    this: *mut FileNavigationContext,
) -> NavigateResult {
    let this = &mut *this;

    if this.requirer_path == "-" {
        return convert(vfs_navigator_reset_to_std_in(&mut this.vfs));
    }

    let requirer_path = this.requirer_path.clone();
    convert(this.vfs.reset_to_path(&requirer_path))
}
