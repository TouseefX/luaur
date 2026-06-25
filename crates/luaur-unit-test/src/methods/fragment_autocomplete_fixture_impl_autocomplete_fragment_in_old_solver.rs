//! C++ `void FragmentAutocompleteFixtureImpl::autocompleteFragmentInOldSolver(...)`
//! (tests/FragmentAutocomplete.test.cpp:211-228).
use crate::functions::get_options::get_options;
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
use alloc::boxed::Box;
use alloc::string::String;
use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
use luaur_analysis::enums::solver_mode::SolverMode;
use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
use luaur_ast::records::position::Position;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl FragmentAutocompleteFixtureImpl {
    pub fn autocomplete_fragment_in_old_solver(
        &mut self,
        document: &String,
        updated: &String,
        marker: char,
        assertions: Box<dyn Fn(&mut FragmentAutocompleteStatusResult)>,
        fragment_end_position: Option<Position>,
    ) {
        let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);

        let _clean_document = self.clean_markers(document);
        let clean_updated = self.clean_markers(updated);
        let cursor_pos = self.get_position(marker);

        self.base
            .get_frontend()
            .set_luau_solver_mode(SolverMode::Old);
        self.base
            .base
            .check_string_optional_frontend_options(document, Some(get_options()));

        let mut result =
            self.autocomplete_fragment(&clean_updated, cursor_pos, fragment_end_position);
        LUAU_ASSERT!(result.status != FragmentAutocompleteStatus::InternalIce);
        assertions(&mut result);
    }
}
