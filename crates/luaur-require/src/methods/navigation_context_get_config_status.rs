use crate::enums::config_status::ConfigStatus;
use crate::records::navigation_context::NavigationContext;

impl NavigationContext {
    pub fn get_config_status(&self) -> ConfigStatus {
        ConfigStatus::Absent
    }
}
