use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::type_aliases::type_id::TypeId;

impl Fixture {
    pub fn get_type(&mut self, name: &String, for_autocomplete: bool) -> Option<TypeId> {
        let module = self.get_main_module(for_autocomplete);
        if module.is_null() {
            panic!("getType: No main module");
        }

        let module = unsafe { &*module };
        if !module.has_module_scope() {
            return None;
        }

        let scope = module.get_module_scope();
        // C++ `Fixture::getType` (Fixture.cpp:457-469): the new solver reads the
        // raw binding via the free `linearSearchForBinding`; the old solver uses
        // `lookupName` (the `Scope` method). Both ultimately return
        // `binding.typeId`, so the distinction is preserved for fidelity.
        if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
            crate::functions::linear_search_for_binding::linear_search_for_binding(
                scope.as_ref() as *const _ as *mut _,
                name.as_str(),
            )
        } else {
            crate::functions::lookup_name::lookup_name(scope.as_ref() as *const _ as *mut _, name)
        }
    }
}
