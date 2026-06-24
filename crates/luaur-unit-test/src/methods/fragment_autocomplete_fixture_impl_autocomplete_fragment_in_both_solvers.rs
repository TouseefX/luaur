//! C++ `void FragmentAutocompleteFixtureImpl::autocompleteFragmentInBothSolvers(...)`
//! (tests/FragmentAutocomplete.test.cpp:230-258).
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
    pub fn autocomplete_fragment_in_both_solvers(
        &mut self,
        document: &String,
        updated: &String,
        marker: char,
        assertions: Box<dyn Fn(&mut FragmentAutocompleteStatusResult)>,
        fragment_end_position: Option<Position>,
    ) {
        let clean_document = self.clean_markers(document);
        let clean_updated = self.clean_markers(updated);
        let cursor_pos = self.get_position(marker);

        {
            let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
            self.base.get_frontend().set_luau_solver_mode(SolverMode::New);
            self.base
                .base
                .check_string_optional_frontend_options(&clean_document, Some(get_options()));

            let mut result = self.autocomplete_fragment(
                &clean_updated,
                cursor_pos,
                fragment_end_position,
            );
            LUAU_ASSERT!(result.status != FragmentAutocompleteStatus::InternalIce);
            assertions(&mut result);
        }

        {
            let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
            self.base.get_frontend().set_luau_solver_mode(SolverMode::Old);
            self.base
                .base
                .check_string_optional_frontend_options(&clean_document, Some(get_options()));

            let mut result = self.autocomplete_fragment(
                &clean_updated,
                cursor_pos,
                fragment_end_position,
            );
            LUAU_ASSERT!(result.status != FragmentAutocompleteStatus::InternalIce);
            assertions(&mut result);
        }
    }
}
