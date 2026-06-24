use crate::enums::prop_index_type::PropIndexType;
use crate::functions::autocomplete_props_autocomplete_core::autocomplete_props as autocomplete_props_full;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::module::Module;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_node::AstNode;

/// C++ `static void autocompleteProps(...)` (AutocompleteCore.cpp:567-579), the
/// seven-parameter overload that seeds `seen` and forwards to the recursive one.
pub fn autocomplete_props(
    module: &Module,
    type_arena: *mut TypeArena,
    builtin_types: &BuiltinTypes,
    ty: TypeId,
    index_type: PropIndexType,
    nodes: &alloc::vec::Vec<*mut AstNode>,
    result: &mut AutocompleteEntryMap,
) {
    let mut seen: std::collections::HashSet<TypeId> = std::collections::HashSet::new();
    autocomplete_props_full(
        module,
        type_arena,
        builtin_types,
        ty,
        ty,
        index_type,
        nodes,
        result,
        &mut seen,
        None,
    );
}
