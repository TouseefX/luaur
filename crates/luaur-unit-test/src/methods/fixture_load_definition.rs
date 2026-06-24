use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::functions::freeze::freeze;
use luaur_analysis::functions::unfreeze::unfreeze;
use luaur_analysis::records::load_definition_file_result::LoadDefinitionFileResult;

impl Fixture {
    pub fn load_definition(
        &mut self,
        source: &String,
        for_autocomplete: bool,
    ) -> LoadDefinitionFileResult {
        let frontend = self.get_frontend();
        let frontend_ptr = frontend as *mut luaur_analysis::records::frontend::Frontend;

        let result = unsafe {
            let globals = if for_autocomplete {
                &mut (*frontend_ptr).globals_for_autocomplete
            } else {
                &mut (*frontend_ptr).globals
            };

            unfreeze(globals.global_types_mut());
            let target_scope = globals.global_scope();
            let result = (*frontend_ptr).load_definition_file(
                globals,
                target_scope,
                source,
                String::from("@test"),
                false,
                for_autocomplete,
            );
            freeze(globals.global_types_mut());
            result
        };

        assert!(
            result.success,
            "loadDefinition: unable to load definition file"
        );
        result
    }
}
