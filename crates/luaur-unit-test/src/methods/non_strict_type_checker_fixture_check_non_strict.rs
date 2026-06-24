use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
use luaur_analysis::records::check_result::CheckResult;
use luaur_ast::enums::mode::Mode;

impl NonStrictTypeCheckerFixture {
    pub fn check_non_strict(&mut self, code: &String) -> CheckResult {
        let _sff = crate::type_aliases::scoped_fast_flag::ScopedFastFlag::new(
            &luaur_common::FFlag::DebugLuauForceOldSolver,
            false,
        );
        self.get_frontend();
        let definitions = self.definitions.clone();
        let res = self.base.load_definition(&definitions, false);
        assert!(res.success);
        self.base
            .check_mode_string_optional_frontend_options(Mode::Nonstrict, code, None)
    }
}
