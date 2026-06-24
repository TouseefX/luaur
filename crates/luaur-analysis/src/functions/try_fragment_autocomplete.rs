//! C++ `FragmentAutocompleteStatusResult tryFragmentAutocomplete(...)`
//! (FragmentAutocomplete.cpp:1337-1371).
use crate::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
use crate::functions::fragment_autocomplete::fragment_autocomplete;
use crate::functions::is_within_comment_module_alt_c::is_within_comment_parse_result_position;
use crate::functions::is_within_hot_comment_module_alt_c::is_within_hot_comment_parse_result_position;
use crate::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
use crate::records::fragment_context::FragmentContext;
use crate::records::frontend::Frontend;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::string_completion_callback::StringCompletionCallback;
use luaur_ast::records::position::Position;

pub fn try_fragment_autocomplete(
    frontend: &mut Frontend,
    module_name: &ModuleName,
    cursor_position: Position,
    context: FragmentContext,
    string_completion_cb: StringCompletionCallback,
) -> FragmentAutocompleteStatusResult {
    let is_in_hot_comment =
        is_within_hot_comment_parse_result_position(context.fresh_parse, cursor_position);
    let is_in_comment =
        is_within_comment_parse_result_position(context.fresh_parse, cursor_position);

    if is_in_comment && !is_in_hot_comment {
        return FragmentAutocompleteStatusResult {
            status: FragmentAutocompleteStatus::Success,
            result: None,
        };
    }

    // TODO: we should calculate fragmentEnd position here, by using context.newAstRoot and cursorPosition
    // C++ wraps fragmentAutocomplete in try/catch on InternalCompilerError, returning
    // FragmentAutocompleteStatus::InternalIce on failure. The Rust equivalent of a thrown
    // InternalCompilerError is a panic, so we catch it here.
    let recent_parse =
        context.fresh_parse.root as *mut luaur_ast::records::ast_stat_block::AstStatBlock;
    let new_src = context.new_src.clone();
    let opts = context.opts.clone();
    let fragment_end = context.DEPRECATED_fragment_end_position;
    let reporter = context.reporter;

    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        fragment_autocomplete(
            frontend,
            &new_src,
            module_name,
            cursor_position,
            opts,
            string_completion_cb,
            fragment_end,
            recent_parse,
            reporter,
            is_in_hot_comment,
        )
    }));

    match outcome {
        Ok(fragment_autocomplete_result) => FragmentAutocompleteStatusResult {
            status: FragmentAutocompleteStatus::Success,
            result: Some(fragment_autocomplete_result),
        },
        Err(_) => FragmentAutocompleteStatusResult {
            status: FragmentAutocompleteStatus::InternalIce,
            result: None,
        },
    }
}
