use crate::enums::config_behavior::ConfigBehavior;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
    pub fn get_config_behavior(&self) -> ConfigBehavior {
        unsafe {
            if (*self.config).get_alias.is_some() {
                ConfigBehavior::GetAlias
            } else {
                ConfigBehavior::GetConfig
            }
        }
    }
}
