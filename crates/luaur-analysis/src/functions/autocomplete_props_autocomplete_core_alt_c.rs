use crate::enums::prop_index_type::PropIndexType;
use crate::functions::autocomplete_props_autocomplete_core_alt_b::autocomplete_props as autocomplete_props_seed;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::module::Module;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_node::AstNode;

/// C++ `AutocompleteEntryMap autocompleteProps(...)` (AutocompleteCore.cpp:581-593),
/// the public six-parameter overload returning a fresh map.
pub fn autocomplete_props(
    module: &Module,
    type_arena: *mut TypeArena,
    builtin_types: &BuiltinTypes,
    ty: TypeId,
    index_type: PropIndexType,
    nodes: &alloc::vec::Vec<*mut AstNode>,
) -> AutocompleteEntryMap {
    let mut result: AutocompleteEntryMap = Default::default();
    autocomplete_props_seed(
        module,
        type_arena,
        builtin_types,
        ty,
        index_type,
        nodes,
        &mut result,
    );
    result
}
