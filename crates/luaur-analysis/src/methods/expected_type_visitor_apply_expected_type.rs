//! @interface-stub
use crate::functions::extract_matching_table_type::extract_matching_table_type;
use crate::functions::extract_matching_table_type_deprecated::extract_matching_table_type_deprecated;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_record::is_record;
use crate::records::expected_type_visitor::ExpectedTypeVisitor;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_type::TableType;
use crate::records::union_type::UnionType;
use crate::type_aliases::singleton_variant::SingletonVariant;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;
use luaur_ast::records::ast_expr_table::{AstExprTable, ItemKind};
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as_const;

impl ExpectedTypeVisitor {
    pub fn apply_expected_type(&mut self, expected_type: TypeId, expr: *const AstExpr) {
        unsafe {
            let expected_type = follow_type_id(expected_type);

            // No matter what, we set the expected type of the current expression to
            // whatever was just passed in. We may traverse the type and do more.
            *(*self.ast_expected_types).get_or_insert(expr) = expected_type;

            let expr_table = ast_node_as_const::<AstExprTable>(expr as *const AstNode);
            if !expr_table.is_null() {
                let expr_table = &*expr_table;

                let expected_table_type = get_type_id::<TableType>(expected_type);
                if expected_table_type.is_null() {
                    let utv = get_type_id::<UnionType>(expected_type);
                    if !utv.is_null() {
                        if let Some(&expr_type) = (*self.ast_types).find(&expr) {
                            if luaur_common::FFlag::LuauBidirectionalInferenceBetterUnionHandling
                                .get()
                            {
                                if let Some(tt) = extract_matching_table_type(
                                    &*utv,
                                    expr_type,
                                    self.builtin_types,
                                ) {
                                    self.apply_expected_type(tt, expr);
                                    return;
                                }
                            } else {
                                let mut parts: Vec<TypeId> = (*utv).options.clone();
                                if let Some(tt) = extract_matching_table_type_deprecated(
                                    &mut parts,
                                    expr_type,
                                    self.builtin_types,
                                ) {
                                    self.apply_expected_type(tt, expr);
                                    return;
                                }
                            }
                        }
                    }
                    return;
                }

                let expected_table_type = &*expected_table_type;

                // If we have a table, then the expected type for any given key is a
                // union between all the possible keys and an indexer type (if it exists).
                let mut possible_key_types: Vec<TypeId> = Vec::new();
                possible_key_types.reserve(
                    expected_table_type.props.len()
                        + if expected_table_type.indexer.is_some() {
                            1
                        } else {
                            0
                        },
                );
                for (name, _) in expected_table_type.props.iter() {
                    possible_key_types.push((*self.arena).add_type(SingletonType::singleton_type(
                        SingletonVariant::V1(StringSingleton::new(name.clone())),
                    )));
                }

                if let Some(indexer) = &expected_table_type.indexer {
                    possible_key_types.push(indexer.index_type);
                }

                let expected_key_type: TypeId = if possible_key_types.is_empty() {
                    (*self.builtin_types).neverType
                } else if possible_key_types.len() == 1 {
                    possible_key_types[0]
                } else {
                    (*self.arena).add_type(UnionType {
                        options: core::mem::take(&mut possible_key_types),
                    })
                };

                for idx in 0..expr_table.items.size {
                    let item = &*expr_table.items.data.add(idx);
                    if is_record(item) {
                        let key_const_string =
                            ast_node_as_const::<AstExprConstantString>(item.key as *const AstNode);
                        if key_const_string.is_null() {
                            continue;
                        }
                        let s = &(*key_const_string).value;
                        let slice =
                            core::slice::from_raw_parts(s.data as *const u8, s.size as usize);
                        let key_str = core::str::from_utf8_unchecked(slice).to_string();

                        // No mater what, we can claim that the expected key type is the
                        // union of all possible props plus the indexer.
                        self.apply_expected_type(expected_key_type, item.key as *const _);

                        // - If the property is defined and has a read type, apply it
                        //   as an expected type. e.g.:
                        //
                        //      -- _ will have expected type `number`
                        //      local t: { [string]: number, write foo: boolean } = { foo = _ }
                        //
                        // - Otherwise if the property has an indexer, apply the result type.
                        // - Otherwise do nothing.
                        if let Some(prop) = expected_table_type.props.get(&key_str) {
                            if let Some(read_ty) = prop.read_ty {
                                self.apply_expected_type(read_ty, item.value as *const _);
                            } else if let Some(indexer) = &expected_table_type.indexer {
                                self.apply_expected_type(
                                    indexer.index_result_type,
                                    item.value as *const _,
                                );
                            }
                        } else if let Some(indexer) = &expected_table_type.indexer {
                            self.apply_expected_type(
                                indexer.index_result_type,
                                item.value as *const _,
                            );
                        }
                    } else if item.kind == ItemKind::List && expected_table_type.indexer.is_some() {
                        let indexer = expected_table_type.indexer.as_ref().unwrap();
                        self.apply_expected_type(indexer.index_result_type, item.value as *const _);
                    } else if item.kind == ItemKind::General
                        && expected_table_type.indexer.is_some()
                    {
                        let indexer = expected_table_type.indexer.as_ref().unwrap();
                        self.apply_expected_type(indexer.index_result_type, item.value as *const _);
                        self.apply_expected_type(expected_key_type, item.key as *const _);
                    }
                }
            } else {
                let group = ast_node_as_const::<AstExprGroup>(expr as *const AstNode);
                if !group.is_null() {
                    self.apply_expected_type(expected_type, (*group).expr as *const _);
                } else {
                    let ternary = ast_node_as_const::<AstExprIfElse>(expr as *const AstNode);
                    if !ternary.is_null() {
                        self.apply_expected_type(expected_type, (*ternary).true_expr as *const _);
                        self.apply_expected_type(expected_type, (*ternary).false_expr as *const _);
                    }
                }
            }
        }
    }
}
