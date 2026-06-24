//! Ported from `tests/Frontend.test.cpp`.
use crate::records::frontend_fixture::FrontendFixture;
use luaur_analysis::functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions;
use luaur_analysis::records::frontend::Frontend;

impl FrontendFixture {
    pub fn get_frontend(&mut self) -> &mut Frontend {
        let already_initialized = self.base.base.frontend.is_some();
        let frontend_ptr = self.base.get_frontend() as *mut Frontend;

        if !already_initialized {
            unsafe {
                let any_type = (*(*frontend_ptr).builtin_types).anyType;
                add_global_binding_builtin_definitions(
                    &mut (*frontend_ptr).globals,
                    "game",
                    any_type,
                    "@test",
                );
                add_global_binding_builtin_definitions(
                    &mut (*frontend_ptr).globals,
                    "script",
                    any_type,
                    "@test",
                );
            }
        }

        unsafe { &mut *frontend_ptr }
    }
}
