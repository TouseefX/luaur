use crate::records::file_navigation_context::FileNavigationContext;
use luaur_require::enums::config_behavior::ConfigBehavior;

#[allow(non_snake_case)]
pub unsafe fn file_navigation_context_get_config_behavior(
    _this: *const FileNavigationContext,
) -> ConfigBehavior {
    ConfigBehavior::GetConfig
}
