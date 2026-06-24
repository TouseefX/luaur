use crate::functions::get_options::get_options;
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
use alloc::string::String;
use luaur_analysis::records::check_result::CheckResult;
use luaur_ast::enums::mode::Mode;
use luaur_common::FFlag;

impl FragmentAutocompleteFixtureImpl {
    pub fn check_old_solver(&mut self, source: &String) -> CheckResult {
        let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
        self.base
            .base
            .check_mode_string_optional_frontend_options(Mode::Strict, source, Some(get_options()))
    }
}
