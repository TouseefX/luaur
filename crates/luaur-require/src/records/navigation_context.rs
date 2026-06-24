use crate::enums::navigate_result::NavigateResult;
use crate::functions::convert_navigate_result::convert_navigate_result;
use crate::records::runtime_luau_config_timer::RuntimeLuauConfigTimer;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;
use alloc::string::String;
use alloc::{ffi::CString, rc::Rc};
use core::ffi::{c_int, c_void};
use core::option::Option;
use luaur_vm::functions::lua_getthreaddata::lua_getthreaddata;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_setthreaddata::lua_setthreaddata;
use luaur_vm::type_aliases::lua_state::lua_State;

pub use crate::enums::config_behavior::ConfigBehavior;
pub use crate::enums::config_status::ConfigStatus;

#[allow(non_camel_case_types)]
pub struct NavigationContext {
    pub(crate) luau_config_init: Option<alloc::boxed::Box<dyn Fn(*mut core::ffi::c_void)>>,
    pub(crate) luau_config_interrupt:
        Option<unsafe extern "C-unwind" fn(l: *mut core::ffi::c_void, gc: core::ffi::c_int)>,
}

impl NavigationContext {
    pub const NavigateResult: () = ();
    pub const ConfigBehavior: () = ();
    pub const ConfigStatus: () = ();
}

pub trait NavigationContextTrait {
    fn reset_to_requirer(&mut self) -> NavigateResult;
    fn jump_to_alias(&mut self, path: &str) -> NavigateResult;

    fn to_alias_override(&mut self, _alias_unprefixed: &str) -> NavigateResult {
        NavigateResult::NotFound
    }

    fn to_alias_fallback(&mut self, _alias_unprefixed: &str) -> NavigateResult {
        NavigateResult::NotFound
    }

    fn to_parent(&mut self) -> NavigateResult;
    fn to_child(&mut self, component: &str) -> NavigateResult;

    fn get_config_status(&self) -> ConfigStatus {
        ConfigStatus::Absent
    }

    fn get_config_behavior(&self) -> ConfigBehavior {
        ConfigBehavior::GetAlias
    }

    fn get_alias(&self, _alias: &str) -> Option<String> {
        None
    }

    fn get_config(&self) -> Option<String> {
        None
    }

    fn luau_config_init(&self) -> Option<Rc<dyn Fn(*mut lua_State)>> {
        None
    }

    fn luau_config_interrupt(
        &self,
    ) -> Option<unsafe extern "C-unwind" fn(l: *mut lua_State, gc: c_int)> {
        None
    }
}

impl NavigationContextTrait for NavigationContext {
    fn reset_to_requirer(&mut self) -> NavigateResult {
        NavigateResult::NotFound
    }

    fn jump_to_alias(&mut self, _path: &str) -> NavigateResult {
        NavigateResult::NotFound
    }

    fn to_parent(&mut self) -> NavigateResult {
        NavigateResult::NotFound
    }

    fn to_child(&mut self, _component: &str) -> NavigateResult {
        NavigateResult::NotFound
    }
}

impl NavigationContextTrait for RuntimeNavigationContext {
    fn reset_to_requirer(&mut self) -> NavigateResult {
        unsafe {
            if self.config.is_null() {
                return NavigateResult::NotFound;
            }

            let Some(reset) = (*self.config).reset else {
                return NavigateResult::NotFound;
            };

            let Ok(requirer_chunkname) = CString::new(self.requirer_chunkname.as_str()) else {
                return NavigateResult::NotFound;
            };

            convert_navigate_result(reset(self.l, self.ctx, requirer_chunkname.as_ptr()) as i32)
        }
    }

    fn jump_to_alias(&mut self, path: &str) -> NavigateResult {
        unsafe {
            if self.config.is_null() {
                return NavigateResult::NotFound;
            }

            let Some(jump_to_alias) = (*self.config).jump_to_alias else {
                return NavigateResult::NotFound;
            };

            let Ok(path) = CString::new(path) else {
                return NavigateResult::NotFound;
            };

            convert_navigate_result(jump_to_alias(self.l, self.ctx, path.as_ptr()) as i32)
        }
    }

    fn to_alias_override(&mut self, alias_unprefixed: &str) -> NavigateResult {
        unsafe {
            if self.config.is_null() {
                return NavigateResult::NotFound;
            }

            let Some(to_alias_override) = (*self.config).to_alias_override else {
                return NavigateResult::NotFound;
            };

            let Ok(alias_unprefixed) = CString::new(alias_unprefixed) else {
                return NavigateResult::NotFound;
            };

            convert_navigate_result(
                to_alias_override(self.l, self.ctx, alias_unprefixed.as_ptr()) as i32,
            )
        }
    }

    fn to_alias_fallback(&mut self, alias_unprefixed: &str) -> NavigateResult {
        unsafe {
            if self.config.is_null() {
                return NavigateResult::NotFound;
            }

            let Some(to_alias_fallback) = (*self.config).to_alias_fallback else {
                return NavigateResult::NotFound;
            };

            let Ok(alias_unprefixed) = CString::new(alias_unprefixed) else {
                return NavigateResult::NotFound;
            };

            convert_navigate_result(
                to_alias_fallback(self.l, self.ctx, alias_unprefixed.as_ptr()) as i32,
            )
        }
    }

    fn to_parent(&mut self) -> NavigateResult {
        RuntimeNavigationContext::to_parent(self)
    }

    fn to_child(&mut self, component: &str) -> NavigateResult {
        unsafe {
            if self.config.is_null() {
                return NavigateResult::NotFound;
            }

            let Some(to_child) = (*self.config).to_child else {
                return NavigateResult::NotFound;
            };

            let Ok(component) = CString::new(component) else {
                return NavigateResult::NotFound;
            };

            convert_navigate_result(to_child(self.l, self.ctx, component.as_ptr()) as i32)
        }
    }

    fn get_config_status(&self) -> ConfigStatus {
        RuntimeNavigationContext::get_config_status(self)
    }

    fn get_config_behavior(&self) -> ConfigBehavior {
        RuntimeNavigationContext::get_config_behavior(self)
    }

    fn get_alias(&self, alias: &str) -> Option<String> {
        RuntimeNavigationContext::get_alias(self, alias)
    }

    fn get_config(&self) -> Option<String> {
        RuntimeNavigationContext::get_config(self)
    }

    fn luau_config_init(&self) -> Option<Rc<dyn Fn(*mut lua_State)>> {
        let config = self.config;
        let ctx = self.ctx;
        let timer = core::ptr::addr_of!(self.timer) as *mut RuntimeLuauConfigTimer as usize;

        Some(Rc::new(move |l: *mut lua_State| unsafe {
            let timeout = if !config.is_null() {
                if let Some(get_timeout) = (*config).get_luau_config_timeout {
                    get_timeout(l as *mut c_void, ctx)
                } else {
                    2000
                }
            } else {
                2000
            };

            let timer = timer as *mut RuntimeLuauConfigTimer;
            (*timer).start(timeout);
            lua_setthreaddata(l, timer as *mut c_void);
        }))
    }

    fn luau_config_interrupt(
        &self,
    ) -> Option<unsafe extern "C-unwind" fn(l: *mut lua_State, gc: c_int)> {
        Some(runtime_luau_config_interrupt)
    }
}

unsafe extern "C-unwind" fn runtime_luau_config_interrupt(l: *mut lua_State, _gc: c_int) {
    let timer = lua_getthreaddata(l) as *const RuntimeLuauConfigTimer;
    if !timer.is_null() && (*timer).is_finished() {
        lua_l_error_l(
            l,
            c"configuration execution timed out".as_ptr(),
            format_args!("configuration execution timed out"),
        );
    }
}
