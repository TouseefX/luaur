use crate::enums::autocomplete_entry_kind::AutocompleteEntryKind;
use crate::enums::type_correct_kind::TypeCorrectKind;
use crate::functions::autocomplete_keywords::autocomplete_keywords;
use crate::functions::get_paren_recommendation::get_paren_recommendation;
use crate::functions::is_binding_legal_at_current_position::is_binding_legal_at_current_position;
use crate::functions::is_identifier::is_identifier;
use crate::functions::is_in_local_names::is_in_local_names;
use crate::functions::is_valid_break_continue_context::is_valid_break_continue_context;
use crate::functions::to_string_symbol::to_string_symbol;
use crate::records::autocomplete_entry::AutocompleteEntry;
use crate::records::binding::Binding;
use crate::records::module::Module;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::scope_ptr_type::ScopePtr;

use alloc::collections::BTreeMap;
use alloc::string::String;

use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_error::AstStatError;
use luaur_ast::records::ast_stat_for::AstStatFor;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;
use luaur_ast::records::ast_stat_while::AstStatWhile;

use luaur_ast::records::position::Position;

use luaur_ast::rtti::ast_node_as;
use luaur_ast::rtti::ast_node_is;
use luaur_common::FFlag;

#[allow(dead_code)]
const kStatementStartingKeywords_DEPRECATED: [&str; 12] = [
    "while", "if", "local", "repeat", "function", "do", "for", "return", "break", "continue",
    "type", "export",
];
const K_STATEMENT_STARTING_KEYWORDS_DEPRECATED: [&str; 12] = [
    "while", "if", "local", "repeat", "function", "do", "for", "return", "break", "continue",
    "type", "export",
];

#[allow(dead_code)]
const kStatementStartingKeywords_CONST: [&str; 13] = [
    "while", "if", "local", "repeat", "function", "do", "for", "return", "break", "continue",
    "type", "export", "const",
];
const K_STATEMENT_STARTING_KEYWORDS_CONST: [&str; 13] = [
    "while", "if", "local", "repeat", "function", "do", "for", "return", "break", "continue",
    "type", "export", "const",
];

#[allow(dead_code)]
const kStatementStartingKeywords_EXPORT: [&str; 14] = [
    "while", "if", "local", "repeat", "function", "do", "for", "return", "break", "continue",
    "type", "export", "const", "export",
];
const K_STATEMENT_STARTING_KEYWORDS_EXPORT: [&str; 14] = [
    "while", "if", "local", "repeat", "function", "do", "for", "return", "break", "continue",
    "type", "export", "const", "export",
];

pub fn autocomplete_statement(
    module: &Module,
    ancestry: &alloc::vec::Vec<*mut AstNode>,
    scope_at_position: &ScopePtr,
    position: &mut Position,
) -> AutocompleteEntryMap {
    let mut result: AutocompleteEntryMap = BTreeMap::new();

    if is_in_local_names(ancestry, *position) {
        autocomplete_keywords(ancestry, *position, &mut result);
        return result;
    }

    let mut scope = Some(scope_at_position.clone());
    while let Some(scope_ref) = scope {
        for (name, binding) in &scope_ref.bindings {
            if !is_binding_legal_at_current_position(name, binding, *position) {
                continue;
            }

            let n = unsafe { to_string_symbol(name) };
            if !result.contains_key(&n) {
                result.insert(
                    n.clone(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Binding,
                        r#type: Some(binding.type_id),
                        deprecated: binding.deprecated,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
                        containing_extern_type: None,
                        prop: None,
                        documentation_symbol: binding.documentation_symbol.clone(),
                        tags: Default::default(),
                        parens: get_paren_recommendation(
                            binding.type_id,
                            ancestry,
                            TypeCorrectKind::None,
                        ),
                        insert_text: None,
                        indexed_with_self: false,
                    },
                );
            }
        }

        scope = scope_ref.parent.clone();
    }

    let should_include_break_and_continue = is_valid_break_continue_context(ancestry, *position);

    if FFlag::LuauExportValueSyntax.get() && FFlag::LuauAutocompleteExport.get() {
        for &kw in &K_STATEMENT_STARTING_KEYWORDS_EXPORT {
            if (kw != "break" && kw != "continue") || should_include_break_and_continue {
                result.insert(
                    kw.to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        }
    } else if FFlag::LuauAutocompleteConst.get() {
        for &kw in &K_STATEMENT_STARTING_KEYWORDS_CONST {
            if (kw != "break" && kw != "continue") || should_include_break_and_continue {
                result.insert(
                    kw.to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        }
    } else {
        for &kw in &K_STATEMENT_STARTING_KEYWORDS_DEPRECATED {
            if (kw != "break" && kw != "continue") || should_include_break_and_continue {
                result.insert(
                    kw.to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        }
    }

    for it_idx in (0..ancestry.len()).rev() {
        let node_ptr = ancestry[it_idx];
        let ast_node = node_ptr as *mut AstNode;

        if unsafe { ast_node_is::<AstStatForIn>(&*ast_node) }
            && !unsafe { (*ast_node_as::<AstStatForIn>(ast_node)).body }.is_null()
        {
            let stat_for_in = unsafe { ast_node_as::<AstStatForIn>(ast_node) };
            if !unsafe { (*(*stat_for_in).body).has_end } {
                result.insert(
                    "end".to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        } else if unsafe { ast_node_is::<AstStatFor>(&*ast_node) } {
            let stat_for = unsafe { ast_node_as::<AstStatFor>(ast_node) };
            if !unsafe { (*(*stat_for).body).has_end } {
                result.insert(
                    "end".to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        } else if unsafe { ast_node_is::<AstStatIf>(&*ast_node) } {
            let stat_if = unsafe { ast_node_as::<AstStatIf>(ast_node) };
            let mut has_end = unsafe { (*stat_if).thenbody }.is_null()
                || unsafe { (*(*stat_if).thenbody).has_end };
            if !unsafe { (*stat_if).elsebody }.is_null() {
                let elsebody = unsafe { (*stat_if).elsebody };
                let elsebody_node = elsebody as *mut AstNode;
                if unsafe { ast_node_is::<AstStatBlock>(&*elsebody_node) } {
                    let else_block = unsafe { ast_node_as::<AstStatBlock>(elsebody_node) };
                    has_end = unsafe { (*else_block).has_end };
                }
            }
            if !has_end {
                result.insert(
                    "end".to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        } else if unsafe { ast_node_is::<AstStatWhile>(&*ast_node) } {
            let stat_while = unsafe { ast_node_as::<AstStatWhile>(ast_node) };
            if !unsafe { (*(*stat_while).body).has_end } {
                result.insert(
                    "end".to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        } else if unsafe { ast_node_is::<AstExprFunction>(&*ast_node) } {
            let expr_function = unsafe { ast_node_as::<AstExprFunction>(ast_node) };
            if !unsafe { (*expr_function).body }.is_null()
                && !unsafe { (*(*expr_function).body).has_end }
            {
                result.insert(
                    "end".to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        }

        if unsafe { ast_node_is::<AstStatBlock>(&*ast_node) } {
            let expr_block = unsafe { ast_node_as::<AstStatBlock>(ast_node) };
            if !unsafe { (*expr_block).has_end } {
                result.insert(
                    "end".to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        }
    }

    if ancestry.len() >= 2 {
        let parent = ancestry[ancestry.len() - 2];
        if unsafe { ast_node_is::<AstStatIf>(&*parent) } {
            let stat_if = unsafe { ast_node_as::<AstStatIf>(parent) };
            let elsebody = unsafe { (*stat_if).elsebody };
            let else_location = unsafe { (*stat_if).else_location };
            if elsebody.is_null()
                || (else_location.is_some() && else_location.unwrap().containsClosed(*position))
            {
                result.insert(
                    "else".to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
                        containing_extern_type: None,
                        prop: None,
                        documentation_symbol: None,
                        tags: Default::default(),
                        parens: Default::default(),
                        insert_text: None,
                        indexed_with_self: false,
                    },
                );
                result.insert(
                    "elseif".to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        }

        if unsafe { ast_node_is::<AstStatRepeat>(&*parent) } {
            let stat_repeat = unsafe { ast_node_as::<AstStatRepeat>(parent) };
            if !unsafe { (*(*stat_repeat).body).has_end } {
                result.insert(
                    "until".to_string(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Keyword,
                        r#type: None,
                        deprecated: false,
                        wrong_index_type: false,
                        type_correct: TypeCorrectKind::None,
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
        }
    }

    if ancestry.len() >= 4 {
        let iter3 = ancestry[ancestry.len() - 4];
        let stat_if_ptr = unsafe {
            if ast_node_is::<AstStatIf>(&*iter3.cast::<AstNode>()) {
                ast_node_as::<AstStatIf>(iter3)
            } else {
                core::ptr::null_mut()
            }
        };
        if !stat_if_ptr.is_null()
            && unsafe { (*stat_if_ptr).elsebody }.is_null()
            && ancestry[ancestry.len() - 3].is_null() == false
            && unsafe { ast_node_is::<AstStatBlock>(&*ancestry[ancestry.len() - 3]) }
            && unsafe { ast_node_is::<AstStatError>(&*ancestry[ancestry.len() - 2]) }
            && is_identifier(ancestry[ancestry.len() - 1])
        {
            result.insert(
                "else".to_string(),
                AutocompleteEntry {
                    kind: AutocompleteEntryKind::Keyword,
                    r#type: None,
                    deprecated: false,
                    wrong_index_type: false,
                    type_correct: TypeCorrectKind::None,
                    containing_extern_type: None,
                    prop: None,
                    documentation_symbol: None,
                    tags: Default::default(),
                    parens: Default::default(),
                    insert_text: None,
                    indexed_with_self: false,
                },
            );
            result.insert(
                "elseif".to_string(),
                AutocompleteEntry {
                    kind: AutocompleteEntryKind::Keyword,
                    r#type: None,
                    deprecated: false,
                    wrong_index_type: false,
                    type_correct: TypeCorrectKind::None,
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
    }

    // extractStat<AstStatRepeat>(ancestry) isn't available here via required context,
    // so mimic by scanning for the first AstStatRepeat from end.
    let mut found_repeat: *mut AstStatRepeat = core::ptr::null_mut();
    for &node in ancestry.iter().rev() {
        if unsafe { ast_node_is::<AstStatRepeat>(&*node) } {
            found_repeat = unsafe { ast_node_as::<AstStatRepeat>(node) };
            break;
        }
    }
    if !found_repeat.is_null() && !unsafe { (*(*found_repeat).body).has_end } {
        result.insert(
            "until".to_string(),
            AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                r#type: None,
                deprecated: false,
                wrong_index_type: false,
                type_correct: TypeCorrectKind::None,
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

    result
}
