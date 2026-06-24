use crate::enums::autocomplete_context::AutocompleteContext;
use crate::enums::autocomplete_entry_kind::AutocompleteEntryKind;
use crate::enums::prop_index_type::PropIndexType;
use crate::enums::type_correct_kind::TypeCorrectKind;
use crate::functions::autocomplete_if_else_expression::autocomplete_if_else_expression;
use crate::functions::autocomplete_props_autocomplete_core_alt_b::autocomplete_props as autocomplete_props_seed;
use crate::functions::autocomplete_string_singleton::autocomplete_string_singleton;
use crate::functions::check_type_correct_kind::check_type_correct_kind;
use crate::functions::find_expected_type_at::find_expected_type_at;
use crate::functions::function_is_expected_at::function_is_expected_at;
use crate::functions::get_paren_recommendation::get_paren_recommendation;
use crate::functions::is_being_defined::is_being_defined;
use crate::functions::is_binding_legal_at_current_position::is_binding_legal_at_current_position;
use crate::functions::to_string_symbol::to_string_symbol;
use crate::records::autocomplete_entry::AutocompleteEntry;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module::Module;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::{ast_node_as, ast_node_is};
use luaur_common::FFlag;

/// C++ `static AutocompleteContext autocompleteExpression(...)`
/// (AutocompleteCore.cpp:1478-1566).
pub fn autocomplete_expression(
    module: &Module,
    builtin_types: &BuiltinTypes,
    type_arena: *mut TypeArena,
    ancestry: &alloc::vec::Vec<*mut AstNode>,
    scope_at_position: &ScopePtr,
    position: Position,
    result: &mut AutocompleteEntryMap,
) -> AutocompleteContext {
    luaur_common::macros::luau_assert::LUAU_ASSERT!(!ancestry.is_empty());

    let node: *mut AstNode = ancestry[ancestry.len() - 1];

    if FFlag::DebugLuauMagicVariableNames.get() {
        let ice = InternalErrorReporter {
            on_internal_error: None,
            module_name: alloc::string::String::new(),
        };
        let local = unsafe { ast_node_as::<AstExprLocal>(node) };
        if !local.is_null()
            && unsafe {
                (*(*local).local)
                    .name
                    .operator_eq_c_char(c"_luau_autocomplete_ice".as_ptr())
            }
        {
            ice.ice_string_location("_luau_autocomplete_ice encountered", unsafe {
                &(*local).base.base.location
            });
        }
        let global = unsafe { ast_node_as::<AstExprGlobal>(node) };
        if !global.is_null()
            && unsafe {
                (*global)
                    .name
                    .operator_eq_c_char(c"_luau_autocomplete_ice".as_ptr())
            }
        {
            ice.ice_string_location("_luau_autocomplete_ice encountered", unsafe {
                &(*global).base.base.location
            });
        }
    }

    if unsafe { ast_node_is::<AstExprIndexName>(&*node) } {
        let expr = unsafe { (*node).as_expr_const() as *const AstExpr };
        if let Some(it) = module.ast_types.find(&expr) {
            autocomplete_props_seed(
                module,
                type_arena,
                builtin_types,
                *it,
                PropIndexType::Point,
                ancestry,
                result,
            );
        }
    } else if autocomplete_if_else_expression(
        node as *const AstNode,
        &mut ancestry.clone(),
        position,
        result,
    ) {
        return AutocompleteContext::Keyword;
    } else if unsafe { ast_node_is::<AstExprFunction>(&*node) } {
        return AutocompleteContext::Unknown;
    } else {
        // This is inefficient. :(
        let mut scope: Option<ScopePtr> = Some(scope_at_position.clone());

        while let Some(scope_ref) = scope {
            for (name, binding) in &scope_ref.bindings {
                if !is_binding_legal_at_current_position(name, binding, position) {
                    continue;
                }

                if is_being_defined(ancestry, name) {
                    continue;
                }

                let n = unsafe { to_string_symbol(name) };
                if !result.contains_key(&n) {
                    let type_correct = check_type_correct_kind(
                        module,
                        type_arena,
                        builtin_types,
                        node,
                        position,
                        binding.type_id,
                    );

                    result.insert(
                        n.clone(),
                        AutocompleteEntry {
                            kind: AutocompleteEntryKind::Binding,
                            r#type: Some(binding.type_id),
                            deprecated: binding.deprecated,
                            wrong_index_type: false,
                            type_correct,
                            containing_extern_type: None,
                            prop: None,
                            documentation_symbol: binding.documentation_symbol.clone(),
                            tags: Default::default(),
                            parens: get_paren_recommendation(
                                binding.type_id,
                                ancestry,
                                type_correct,
                            ),
                            insert_text: None,
                            indexed_with_self: false,
                        },
                    );
                }
            }

            scope = scope_ref.parent.clone();
        }

        let correct_for_nil = check_type_correct_kind(
            module,
            type_arena,
            builtin_types,
            node,
            position,
            builtin_types.nilType,
        );
        let correct_for_true = check_type_correct_kind(
            module,
            type_arena,
            builtin_types,
            node,
            position,
            builtin_types.trueType,
        );
        let correct_for_false = check_type_correct_kind(
            module,
            type_arena,
            builtin_types,
            node,
            position,
            builtin_types.falseType,
        );
        let correct_for_function =
            if function_is_expected_at(module, node, position).unwrap_or(false) {
                TypeCorrectKind::Correct
            } else {
                TypeCorrectKind::None
            };

        result.insert(
            alloc::string::String::from("if"),
            AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                r#type: None,
                deprecated: false,
                wrong_index_type: false,
                ..Default::default()
            },
        );
        result.insert(
            alloc::string::String::from("true"),
            AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                r#type: Some(builtin_types.booleanType),
                deprecated: false,
                wrong_index_type: false,
                type_correct: correct_for_true,
                ..Default::default()
            },
        );
        result.insert(
            alloc::string::String::from("false"),
            AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                r#type: Some(builtin_types.booleanType),
                deprecated: false,
                wrong_index_type: false,
                type_correct: correct_for_false,
                ..Default::default()
            },
        );
        result.insert(
            alloc::string::String::from("nil"),
            AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                r#type: Some(builtin_types.nilType),
                deprecated: false,
                wrong_index_type: false,
                type_correct: correct_for_nil,
                ..Default::default()
            },
        );
        result.insert(
            alloc::string::String::from("not"),
            AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                ..Default::default()
            },
        );
        result.insert(
            alloc::string::String::from("function"),
            AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                r#type: None,
                deprecated: false,
                wrong_index_type: false,
                type_correct: correct_for_function,
                ..Default::default()
            },
        );

        if let Some(ty) = find_expected_type_at(module, node, position) {
            autocomplete_string_singleton(ty, true, node, position, result);
        }
    }

    AutocompleteContext::Expression
}
