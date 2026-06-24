//! C++ `FragmentAutocompleteResult fragmentAutocomplete(...)`
//! (FragmentAutocomplete.cpp:1373-1413).
use crate::enums::fragment_type_check_status::FragmentTypeCheckStatus;
use crate::enums::fragment_autocomplete_waypoint::FragmentAutocompleteWaypoint;
use crate::functions::autocomplete_autocomplete_core::autocomplete_;
use crate::functions::report_waypoint::report_waypoint;
use crate::functions::typecheck_fragment_fragment_autocomplete_alt_b::typecheck_fragment;
use crate::functions::unfreeze::unfreeze;
use crate::functions::freeze::freeze;
use crate::records::autocomplete_result::AutocompleteResult;
use crate::records::fragment_autocomplete_result::FragmentAutocompleteResult;
use crate::records::frontend::Frontend;
use crate::records::frontend_options::FrontendOptions;
use crate::records::i_fragment_autocomplete_reporter::IFragmentAutocompleteReporter;
use crate::records::module::Module;
use crate::records::scope::Scope;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::string_completion_callback::StringCompletionCallback;
use alloc::sync::Arc;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::position::Position;
use luaur_common::macros::luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT;
use luaur_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

#[allow(clippy::too_many_arguments)]
pub fn fragment_autocomplete(
    frontend: &mut Frontend,
    src: &str,
    module_name: &ModuleName,
    cursor_position: Position,
    opts: Option<FrontendOptions>,
    callback: StringCompletionCallback,
    fragment_end_position: Option<Position>,
    recent_parse: *mut AstStatBlock,
    reporter: *mut dyn IFragmentAutocompleteReporter,
    is_in_hot_comment: bool,
) -> FragmentAutocompleteResult {
    LUAU_TIMETRACE_SCOPE!("Luau::fragmentAutocomplete", "FragmentAutocomplete");
    LUAU_TIMETRACE_ARGUMENT!("name", module_name.as_str());

    let for_autocomplete = opts.as_ref().map(|o| o.for_autocomplete).unwrap_or(false);

    let (tc_status, mut tc_result) = typecheck_fragment(
        frontend,
        module_name,
        &cursor_position,
        opts,
        src,
        fragment_end_position,
        recent_parse,
        reporter,
    );

    if tc_status == FragmentTypeCheckStatus::SkipAutocomplete {
        // C++ `return {}` — a default FragmentAutocompleteResult.
        return FragmentAutocompleteResult {
            incremental_module: Arc::new(Module::default()),
            fresh_scope: core::ptr::null_mut(),
            ac_results: AutocompleteResult::autocomplete_result(),
        };
    }

    report_waypoint(reporter, FragmentAutocompleteWaypoint::TypecheckFragmentEnd);

    let global_scope = if for_autocomplete {
        Arc::as_ptr(&frontend.globals_for_autocomplete.global_scope) as *mut Scope
    } else {
        Arc::as_ptr(&frontend.globals.global_scope) as *mut Scope
    };

    let incremental_module = tc_result
        .incremental_module
        .take()
        .expect("typecheckFragment must have produced an incremental module on Success");
    let module_ptr = Arc::as_ptr(&incremental_module) as *mut Module;
    let builtin_types = unsafe { &*frontend.builtin_types };
    let fresh_scope = tc_result.fresh_scope.clone();

    unsafe {
        unfreeze(&mut (*module_ptr).internal_types);
    }

    let result = autocomplete_(
        &incremental_module,
        builtin_types,
        unsafe { &mut (*module_ptr).internal_types },
        &mut tc_result.ancestry,
        global_scope,
        &fresh_scope,
        cursor_position,
        frontend.file_resolver,
        callback,
        is_in_hot_comment,
    );

    unsafe {
        freeze(&mut (*module_ptr).internal_types);
    }

    report_waypoint(reporter, FragmentAutocompleteWaypoint::AutocompleteEnd);

    FragmentAutocompleteResult {
        fresh_scope: Arc::as_ptr(&fresh_scope) as *mut Scope,
        incremental_module,
        ac_results: result,
    }
}
