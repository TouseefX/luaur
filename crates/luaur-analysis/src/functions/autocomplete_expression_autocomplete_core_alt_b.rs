use crate::enums::autocomplete_context::AutocompleteContext;
use crate::functions::autocomplete_expression_autocomplete_core::autocomplete_expression as autocomplete_expression_into;
use crate::records::autocomplete_result::AutocompleteResult;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::module::Module;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::position::Position;

/// C++ `static AutocompleteResult autocompleteExpression(...)`
/// (AutocompleteCore.cpp:1568-1580), the six-parameter overload that builds a
/// fresh result.
pub fn autocomplete_expression(
    module: &Module,
    builtin_types: &BuiltinTypes,
    type_arena: *mut TypeArena,
    ancestry: &alloc::vec::Vec<*mut AstNode>,
    scope_at_position: &ScopePtr,
    position: Position,
) -> AutocompleteResult {
    let mut result: AutocompleteEntryMap = Default::default();
    let context: AutocompleteContext = autocomplete_expression_into(
        module,
        builtin_types,
        type_arena,
        ancestry,
        scope_at_position,
        position,
        &mut result,
    );
    AutocompleteResult {
        entry_map: result,
        ancestry: ancestry.clone(),
        context,
    }
}
