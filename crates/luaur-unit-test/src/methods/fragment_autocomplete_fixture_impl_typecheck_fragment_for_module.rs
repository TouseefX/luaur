//! C++ `std::pair<FragmentTypeCheckStatus, FragmentTypeCheckResult>
//! FragmentAutocompleteFixtureImpl::typecheckFragmentForModule(...)`
//! (tests/FragmentAutocomplete.test.cpp:260-268).
use crate::functions::get_options::get_options;
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use alloc::string::String;
use luaur_analysis::enums::fragment_type_check_status::FragmentTypeCheckStatus;
use luaur_analysis::functions::typecheck_fragment_fragment_autocomplete_alt_b::typecheck_fragment;
use luaur_analysis::records::fragment_type_check_result::FragmentTypeCheckResult;
use luaur_analysis::records::i_fragment_autocomplete_reporter::null_reporter;
use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;
use luaur_ast::records::position::Position;

impl FragmentAutocompleteFixtureImpl {
    pub fn typecheck_fragment_for_module(
        &mut self,
        module: &ModuleName,
        document: &String,
        cursor_pos: Position,
        fragment_end_position: Option<Position>,
    ) -> (FragmentTypeCheckStatus, FragmentTypeCheckResult) {
        let pr = self.parse_helper(document.clone());
        let options = get_options();
        let frontend = self.base.get_frontend();
        typecheck_fragment(
            frontend,
            module,
            &cursor_pos,
            Some(options),
            document.as_str(),
            fragment_end_position,
            pr.root,
            null_reporter(),
        )
    }
}
