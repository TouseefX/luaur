use crate::enums::autocomplete_entry_kind::AutocompleteEntryKind;
use crate::enums::type_correct_kind::TypeCorrectKind;
use crate::functions::find_type_element_at_autocomplete_core_alt_c::find_type_element_at_ast_type_type_id_position;
use crate::functions::try_get_type_name_in_scope_autocomplete_core::try_get_type_name_in_scope;
use crate::records::autocomplete_entry::AutocompleteEntry;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::position::Position;

pub fn try_add_type_correct_suggestion(
    result: &mut AutocompleteEntryMap,
    scope: ScopePtr,
    top_type: *mut AstType,
    inferred_type: TypeId,
    position: Position,
) -> bool {
    let ty = if !top_type.is_null() {
        find_type_element_at_ast_type_type_id_position(top_type, inferred_type, position)
    } else {
        Some(inferred_type)
    };

    let Some(ty) = ty else {
        return false;
    };

    if let Some(name) = try_get_type_name_in_scope(scope, ty, false) {
        if let Some(entry) = result.get_mut(&name) {
            entry.type_correct = TypeCorrectKind::Correct;
        } else {
            result.insert(
                name,
                AutocompleteEntry {
                    kind: AutocompleteEntryKind::Type,
                    r#type: Some(ty),
                    deprecated: false,
                    wrong_index_type: false,
                    type_correct: TypeCorrectKind::Correct,
                    containing_extern_type: None,
                    prop: None,
                    documentation_symbol: None,
                    tags: Default::default(),
                    parens: Default::default(),
                    insert_text: None,
                    indexed_with_self: false,
                },
            );
        }

        return true;
    }

    false
}
