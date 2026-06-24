//! Generated skeleton item.
//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Autocomplete.cpp:17:autocomplete`
//! Source: `Analysis/src/Autocomplete.cpp`

use crate::enums::solver_mode::SolverMode;
use crate::functions::autocomplete_autocomplete_core::autocomplete_;
use crate::functions::find_ancestry_at_position_for_autocomplete_ast_query::find_ancestry_at_position_for_autocomplete_source_module_position;
use crate::functions::find_scope_at_position::find_scope_at_position;
use crate::functions::is_within_comment_module_alt_b::is_within_comment_source_module_position;
use crate::functions::is_within_hot_comment_module_alt_b::is_within_hot_comment_source_module_position;
use crate::records::autocomplete_result::AutocompleteResult;
use crate::records::frontend::Frontend;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use crate::type_aliases::string_completion_callback::StringCompletionCallback;
use luaur_ast::records::position::Position;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::macros::luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT;
use luaur_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

pub fn autocomplete(
    frontend: &mut Frontend,
    module_name: &ModuleName,
    position: Position,
    callback: StringCompletionCallback,
) -> AutocompleteResult {
    LUAU_TIMETRACE_SCOPE!("Luau::autocomplete", "Autocomplete");
    LUAU_TIMETRACE_ARGUMENT!("name", module_name.as_str());

    let source_module = frontend.get_source_module(module_name);
    if source_module.is_null() {
        return AutocompleteResult::autocomplete_result();
    }

    // ModulePtr is modeled as a non-null `Arc<Module>`; C++ `if (!module) return {}`
    // guards a null shared_ptr, which cannot occur for the non-null Arc here.
    let module = if frontend.get_luau_solver_mode() == SolverMode::New {
        frontend.module_resolver.get_module(module_name)
    } else {
        frontend
            .module_resolver_for_autocomplete
            .get_module(module_name)
    };

    let builtin_types = unsafe { &*frontend.builtin_types };

    let global_scope = if frontend.get_luau_solver_mode() == SolverMode::New {
        alloc::sync::Arc::as_ptr(&frontend.globals.global_scope)
            as *mut crate::records::scope::Scope
    } else {
        alloc::sync::Arc::as_ptr(&frontend.globals_for_autocomplete.global_scope)
            as *mut crate::records::scope::Scope
    };

    let mut type_arena = TypeArena::default();
    let is_in_hot_comment =
        is_within_hot_comment_source_module_position(unsafe { &*source_module }, position);
    if is_within_comment_source_module_position(unsafe { &*source_module }, position)
        && !is_in_hot_comment
    {
        return AutocompleteResult::autocomplete_result();
    }

    let mut ancestry = find_ancestry_at_position_for_autocomplete_source_module_position(
        unsafe { &*source_module },
        position,
    );
    LUAU_ASSERT!(!ancestry.is_empty());
    // `findScopeAtPosition` returns a (nullable) ScopePtr; modeled here as
    // `Option<ScopePtr>`. `autocomplete_` takes `&ScopePtr`, so unwrap the
    // resolved scope (the empty-scopes corner is degenerate).
    let start_scope = find_scope_at_position(&module, position).unwrap();

    autocomplete_(
        &module,
        builtin_types,
        &mut type_arena,
        &mut ancestry,
        global_scope,
        &start_scope,
        position,
        frontend.file_resolver,
        callback,
        is_in_hot_comment,
    )
}
