//! @interface-stub
use crate::records::ac_fixture_impl::AcFixtureImpl;
use luaur_analysis::records::load_definition_file_result::LoadDefinitionFileResult;

impl AcFixtureImpl {
    pub fn load_definition(&mut self, source: &alloc::string::String) -> LoadDefinitionFileResult {
        self.get_frontend();

        let result = self.base.load_definition(source, true);

        if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
            self.base.load_definition(source, false);
        }

        assert!(
            result.success,
            "loadDefinition: unable to load definition file"
        );
        result
    }
}
