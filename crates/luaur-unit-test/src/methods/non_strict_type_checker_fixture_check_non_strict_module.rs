use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
use luaur_analysis::records::check_result::CheckResult;

impl NonStrictTypeCheckerFixture {
    pub fn check_non_strict_module(&mut self, module_name: &String) -> CheckResult {
        let _sff = crate::type_aliases::scoped_fast_flag::ScopedFastFlag::new(
            &luaur_common::FFlag::DebugLuauForceOldSolver,
            false,
        );
        self.get_frontend();
        let definitions = self.definitions.clone();
        let res = self.base.load_definition(&definitions, false);
        assert!(res.success);
        self.get_frontend()
            .check_module_name_optional_frontend_options(module_name, None)
    }
}
