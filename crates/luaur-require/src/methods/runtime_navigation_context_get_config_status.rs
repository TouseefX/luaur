use crate::enums::config_status::ConfigStatus;
use crate::functions::convert_config_status::convert_config_status;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
    pub fn get_config_status(&self) -> ConfigStatus {
        unsafe {
            let config_ptr = self.config;
            if let Some(get_config_status_fn) = (*config_ptr).get_config_status {
                let status = get_config_status_fn(self.l, self.ctx);
                convert_config_status(status as i32)
            } else {
                ConfigStatus::Absent
            }
        }
    }
}
