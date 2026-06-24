use crate::enums::config_behavior::ConfigBehavior;
use crate::enums::config_status::ConfigStatus;
use crate::enums::navigate_result::NavigateResult;
use crate::records::luarequire_configuration::luarequire_Configuration;
use crate::records::navigation_context::NavigationContext;
use crate::records::runtime_luau_config_timer::RuntimeLuauConfigTimer;
use alloc::string::String;
use core::ffi::c_void;

#[allow(non_camel_case_types)]
pub struct RuntimeNavigationContext {
    pub(crate) base: NavigationContext,
    pub(crate) config: *mut luarequire_Configuration,
    pub(crate) l: *mut c_void,
    pub(crate) ctx: *mut c_void,
    pub(crate) requirer_chunkname: String,
    pub(crate) timer: RuntimeLuauConfigTimer,
}
