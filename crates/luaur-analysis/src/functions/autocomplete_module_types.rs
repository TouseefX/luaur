use crate::enums::autocomplete_entry_kind::AutocompleteEntryKind;
use crate::records::autocomplete_entry::AutocompleteEntry;
use crate::records::module::Module;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use luaur_ast::records::position::Position;

pub fn autocomplete_module_types(
    _module: &Module,
    scope_at_position: &ScopePtr,
    _position: Position,
    module_name: &str,
) -> AutocompleteEntryMap {
    let mut result = AutocompleteEntryMap::new();
    let mut curr = Some(scope_at_position.clone());

    while let Some(scope_ref) = curr {
        if let Some(name_table) = scope_ref.imported_type_bindings.get(module_name) {
            for (name, ty) in name_table {
                let entry = AutocompleteEntry {
                    kind: AutocompleteEntryKind::Type,
                    r#type: Some(ty.r#type),
                    deprecated: false,
                    wrong_index_type: false,
                    type_correct: crate::enums::type_correct_kind::TypeCorrectKind::None,
                    containing_extern_type: None,
                    prop: None,
                    documentation_symbol: None,
                    tags: crate::type_aliases::tags::Tags::default(),
                    parens:
                        crate::enums::parentheses_recommendation::ParenthesesRecommendation::None,
                    insert_text: None,
                    indexed_with_self: false,
                };
                result.insert(name.clone(), entry);
            }
            break;
        }

        curr = scope_ref.parent.clone();
    }

    result
}
