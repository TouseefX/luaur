use crate::enums::config_behavior::ConfigBehavior;
use crate::records::navigation_context::NavigationContext;

impl NavigationContext {
    pub fn get_config_behavior(&self) -> ConfigBehavior {
        ConfigBehavior::GetAlias
    }
}
