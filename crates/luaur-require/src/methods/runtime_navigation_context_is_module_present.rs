use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
    pub fn is_module_present(&self) -> bool {
        unsafe {
            let config_ptr = self.config;
            if let Some(is_module_present_fn) = (*config_ptr).is_module_present {
                is_module_present_fn(self.l, self.ctx)
            } else {
                false
            }
        }
    }
}
