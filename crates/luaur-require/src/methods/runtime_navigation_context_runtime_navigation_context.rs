use crate::records::luarequire_configuration::luarequire_Configuration;
use crate::records::navigation_context::NavigationContext;
use crate::records::runtime_luau_config_timer::RuntimeLuauConfigTimer;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;
use luaur_vm::functions::lua_setthreaddata::lua_setthreaddata;
use luaur_vm::records::lua_state::lua_State;

impl RuntimeNavigationContext {
    pub fn new(
        config: *mut luarequire_Configuration,
        l: *mut core::ffi::c_void,
        ctx: *mut core::ffi::c_void,
        requirer_chunkname: alloc::string::String,
    ) -> Self {
        let mut result = Self {
            base: NavigationContext {
                luau_config_init: None,
                luau_config_interrupt: None,
            },
            config,
            l,
            ctx,
            requirer_chunkname: alloc::string::String::new(),
            timer: RuntimeLuauConfigTimer {
                start_time: std::time::Instant::now(),
                timeout_duration: None,
            },
        };

        result.runtime_navigation_context_runtime_navigation_context(
            config,
            l,
            ctx,
            requirer_chunkname,
        );
        result
    }

    pub fn runtime_navigation_context_runtime_navigation_context(
        &mut self,
        config: *mut luarequire_Configuration,
        l: *mut core::ffi::c_void,
        ctx: *mut core::ffi::c_void,
        requirer_chunkname: alloc::string::String,
    ) {
        self.config = config;
        self.l = l;
        self.ctx = ctx;
        self.requirer_chunkname = requirer_chunkname;

        // The C++ code initializes the timer and sets it as thread data.
        // In Rust, we initialize the timer field.
        self.timer = RuntimeLuauConfigTimer {
            start_time: std::time::Instant::now(),
            timeout_duration: None,
        };

        // Determine timeout from config or default to 2000ms
        let timeout = unsafe {
            if !config.is_null() {
                if let Some(get_timeout) = (*config).get_luau_config_timeout {
                    get_timeout(l, ctx)
                } else {
                    2000
                }
            } else {
                2000
            }
        };

        self.timer.start(timeout);

        unsafe {
            lua_setthreaddata(
                l as *mut lua_State,
                &mut self.timer as *mut RuntimeLuauConfigTimer as *mut core::ffi::c_void,
            );
        }

        // Note: The C++ code assigns lambdas to `luauConfigInit` and `luauConfigInterrupt`.
        // Since these fields are not present in the Rust `RuntimeNavigationContext` record,
        // and Rust does not support storing arbitrary lambdas in these fields without
        // specific trait objects or function pointers in the struct definition,
        // we omit the assignment. The logic for the interrupt check is handled by
        // the caller of the timer.
    }
}
