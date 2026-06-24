use crate::enums::autocomplete_entry_kind::AutocompleteEntryKind;
use crate::enums::type_correct_kind::TypeCorrectKind;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::autocomplete_entry::AutocompleteEntry;
use crate::records::intersection_type::IntersectionType;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::union_type::UnionType;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::{ast_node_as, ast_node_is};
use luaur_common::functions::escape::escape;
use luaur_common::FFlag;

pub fn autocomplete_string_singleton(
    ty: TypeId,
    add_quotes: bool,
    node: *mut AstNode,
    position: Position,
    result: &mut AutocompleteEntryMap,
) {
    unsafe {
        let node_ref = &*node;
        if position == node_ref.location.begin || position == node_ref.location.end {
            let str_node = ast_node_as::<AstExprConstantString>(node);
            if !str_node.is_null() {
                let str_val = &*str_node;
                if str_val.is_quoted() {
                    return;
                }
            }

            if ast_node_is::<AstExprInterpString>(node_ref) {
                return;
            }
        }

        let format_key = |key: &str| {
            if add_quotes {
                format!("\"{}\"", escape(key, false))
            } else {
                escape(key, false)
            }
        };

        let ty = follow_type_id(ty);

        let ss = get_type_id::<SingletonType>(ty);
        if !ss.is_null() {
            let sstv_inner = get_singleton_type::<StringSingleton>(ss);
            if !sstv_inner.is_null() {
                let key = format_key(&(*sstv_inner).value);
                result.entry(key).or_insert_with(|| AutocompleteEntry {
                    kind: AutocompleteEntryKind::String,
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
                });
            }
            return;
        }

        let uty = get_type_id::<UnionType>(ty);
        if !uty.is_null() {
            let utv = &*uty;
            for &el in &utv.options {
                let ss_el = get_type_id::<SingletonType>(el);
                if !ss_el.is_null() {
                    let sstv_el_inner = get_singleton_type::<StringSingleton>(ss_el);
                    if !sstv_el_inner.is_null() {
                        let key = format_key(&(*sstv_el_inner).value);
                        result.entry(key).or_insert_with(|| AutocompleteEntry {
                            kind: AutocompleteEntryKind::String,
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
                        });
                    }
                }
            }
            return;
        }

        let ity = get_type_id::<IntersectionType>(ty);
        if FFlag::LuauAutocompleteStringSingletonIntersection.get() && !ity.is_null() {
            let itv = &*ity;
            for &el in &itv.parts {
                autocomplete_string_singleton(el, add_quotes, node, position, result);
            }
        }
    }
}
