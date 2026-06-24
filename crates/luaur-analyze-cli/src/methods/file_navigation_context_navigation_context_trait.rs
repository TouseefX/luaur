//! `NavigationContextTrait` impl wiring `FileNavigationContext` to the per-method
//! ports of `CLI/src/AnalyzeRequirer.cpp`. The C++ `: NavigationContext` base
//! relationship is realized here, in lieu of struct embedding.

use crate::records::file_navigation_context::FileNavigationContext;
use alloc::rc::Rc;
use alloc::string::String;
use core::ffi::c_int;
use core::ffi::c_void;
use luaur_require::enums::config_behavior::ConfigBehavior;
use luaur_require::enums::navigate_result::NavigateResult;
use luaur_require::records::navigation_context::{ConfigStatus, NavigationContextTrait};
use luaur_vm::functions::lua_setthreaddata::lua_setthreaddata;
use luaur_vm::type_aliases::lua_state::lua_State;

impl NavigationContextTrait for FileNavigationContext {
    fn reset_to_requirer(&mut self) -> NavigateResult {
        unsafe {
            crate::methods::file_navigation_context_reset_to_requirer::file_navigation_context_reset_to_requirer(self)
        }
    }

    fn jump_to_alias(&mut self, path: &str) -> NavigateResult {
        unsafe {
            crate::methods::file_navigation_context_jump_to_alias::file_navigation_context_jump_to_alias(
                self,
                &String::from(path),
            )
        }
    }

    fn to_parent(&mut self) -> NavigateResult {
        unsafe {
            crate::methods::file_navigation_context_to_parent::file_navigation_context_to_parent(
                self,
            )
        }
    }

    fn to_child(&mut self, component: &str) -> NavigateResult {
        unsafe {
            crate::methods::file_navigation_context_to_child::file_navigation_context_to_child(
                self,
                &String::from(component),
            )
        }
    }

    fn get_config_status(&self) -> ConfigStatus {
        unsafe {
            crate::methods::file_navigation_context_get_config_status::file_navigation_context_get_config_status(self)
        }
    }

    fn get_config_behavior(&self) -> ConfigBehavior {
        unsafe {
            crate::methods::file_navigation_context_get_config_behavior::file_navigation_context_get_config_behavior(self)
        }
    }

    fn get_alias(&self, alias: &str) -> Option<String> {
        unsafe {
            crate::methods::file_navigation_context_get_alias::file_navigation_context_get_alias(
                self,
                &String::from(alias),
            )
        }
    }

    fn get_config(&self) -> Option<String> {
        unsafe {
            crate::methods::file_navigation_context_get_config::file_navigation_context_get_config(
                self,
            )
        }
    }

    /// C++ `navigationContext.luauConfigInit = [&info](lua_State* L) { lua_setthreaddata(L, &info); };`
    /// (`CLI/src/Analyze.cpp:194-197`).
    fn luau_config_init(&self) -> Option<Rc<dyn Fn(*mut lua_State)>> {
        let info_ptr = self.interrupt_info.as_ref()?.as_ref()
            as *const crate::records::luau_config_interrupt_info::LuauConfigInterruptInfo
            as *mut c_void;
        Some(Rc::new(move |l: *mut lua_State| unsafe {
            lua_setthreaddata(l, info_ptr);
        }))
    }

    /// C++ `navigationContext.luauConfigInterrupt = [](lua_State* L, int gc) { ... };`
    /// (`CLI/src/Analyze.cpp:198-205`) — identical body to the config-resolver interrupt.
    fn luau_config_interrupt(
        &self,
    ) -> Option<unsafe extern "C-unwind" fn(l: *mut lua_State, gc: c_int)> {
        self.interrupt_info.as_ref()?;
        Some(crate::methods::cli_config_resolver_read_config_rec::luau_config_interrupt)
    }
}
