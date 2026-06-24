use crate::functions::extract_matching_table_type::extract_matching_table_type;
use crate::functions::extract_matching_table_type_deprecated::extract_matching_table_type_deprecated;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_optional::is_optional;
use crate::functions::is_record::is_record;
use crate::functions::simplify_intersection_simplify_alt_b::simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids;
use crate::records::intersection_type::IntersectionType;
use crate::records::missing_properties::{Context as MissingPropertiesContext, MissingProperties};
use crate::records::table_type::TableType;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_ids::TypeIds;
use crate::records::unexpected_array_like_table_item::UnexpectedArrayLikeTableItem;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_error_data::TypeErrorData;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinary_Op};
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;
use luaur_ast::records::ast_expr_table::ItemKind;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as_const;

impl TypeChecker2 {
    pub fn test_potential_literal_is_subtype(
        &mut self,
        expr: *mut AstExpr,
        expected_type: crate::type_aliases::type_id::TypeId,
    ) -> bool {
        unsafe {
            let expr_type = follow_type_id(self.lookup_type(expr));
            let expected_type = follow_type_id(expected_type);

            let group = ast_node_as_const::<AstExprGroup>(expr as *const AstNode);
            if !group.is_null() {
                return self.test_potential_literal_is_subtype(
                    (*group).expr as *mut AstExpr,
                    expected_type,
                );
            }

            let if_else = ast_node_as_const::<AstExprIfElse>(expr as *const AstNode);
            if !if_else.is_null() {
                let mut passes = self.test_potential_literal_is_subtype(
                    (*if_else).true_expr as *mut AstExpr,
                    expected_type,
                );
                passes &= self.test_potential_literal_is_subtype(
                    (*if_else).false_expr as *mut AstExpr,
                    expected_type,
                );
                return passes;
            }

            let bin_expr = ast_node_as_const::<AstExprBinary>(expr as *const AstNode);
            if !bin_expr.is_null() && (*bin_expr).op == AstExprBinary_Op::Or {
                let relaxed_expected_lhs = (*self.module).internal_types.add_type(UnionType {
                    options: alloc::vec![(*self.builtin_types).falsyType, expected_type,],
                });
                let mut passes = self.test_potential_literal_is_subtype(
                    (*bin_expr).left as *mut AstExpr,
                    relaxed_expected_lhs,
                );
                passes &= self.test_potential_literal_is_subtype(
                    (*bin_expr).right as *mut AstExpr,
                    expected_type,
                );
                return passes;
            }

            let expr_table = ast_node_as_const::<luaur_ast::records::ast_expr_table::AstExprTable>(
                expr as *const AstNode,
            );
            let expr_table_type = get_type_id::<TableType>(expr_type);
            let expected_table_type = get_type_id::<TableType>(expected_type);

            if expr_table_type.is_null() || expr_table.is_null() {
                return self.test_is_subtype_type_id_type_id_location(
                    expr_type,
                    expected_type,
                    (*expr).base.location,
                );
            }

            if expected_table_type.is_null() {
                let expected_union = get_type_id::<UnionType>(expected_type);
                if !expected_union.is_null() {
                    if luaur_common::FFlag::LuauBidirectionalInferenceBetterUnionHandling.get() {
                        if let Some(tt) = extract_matching_table_type(
                            &*expected_union,
                            expr_type,
                            self.builtin_types,
                        ) {
                            return self.test_literal_or_ast_type_is_subtype(expr, tt);
                        }
                    } else {
                        let mut parts = (*expected_union).options.clone();
                        if let Some(tt) = extract_matching_table_type_deprecated(
                            &mut parts,
                            expr_type,
                            self.builtin_types,
                        ) {
                            return self.test_potential_literal_is_subtype(expr, tt);
                        }
                    }
                }

                let expected_intersection = get_type_id::<IntersectionType>(expected_type);
                if !expected_intersection.is_null() {
                    let mut parts = TypeIds::type_ids();
                    for &part in &(*expected_intersection).parts {
                        parts.insert_type_id(part);
                    }

                    let simplified =
                        simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids(
                            self.builtin_types,
                            &mut (*self.module).internal_types,
                            parts,
                        )
                        .result;

                    if !get_type_id::<TableType>(simplified).is_null() {
                        return self.test_potential_literal_is_subtype(expr, simplified);
                    }
                }

                return self.test_is_subtype_type_id_type_id_location(
                    expr_type,
                    expected_type,
                    (*expr).base.location,
                );
            }

            let expected_table_type = &*expected_table_type;
            let mut missing_keys: Vec<String> = Vec::new();
            for (name, prop) in expected_table_type.props.iter() {
                if let Some(read_ty) = prop.read_ty {
                    if !is_optional(read_ty) {
                        missing_keys.push(name.clone());
                    }
                }
            }

            let scope = self.find_innermost_scope((*expr).base.location);
            let mut is_array_like = false;
            if let Some(indexer) = &expected_table_type.indexer {
                let result = (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
                    (*self.builtin_types).numberType,
                    indexer.index_type,
                    scope,
                );
                is_array_like = result.is_subtype
                    || self.is_error_suppressing_location_type_id(
                        (*expr).base.location,
                        indexer.index_type,
                    );
            }

            let mut is_subtype = true;
            for item in (*expr_table).items.iter() {
                if is_record(item) {
                    let key_const_string =
                        ast_node_as_const::<AstExprConstantString>(item.key as *const AstNode);
                    if key_const_string.is_null() {
                        continue;
                    }

                    let s = &(*key_const_string).value;
                    let slice = core::slice::from_raw_parts(s.data as *const u8, s.size as usize);
                    let key_str = core::str::from_utf8_unchecked(slice).to_string();

                    missing_keys.retain(|key| key != &key_str);

                    if let Some(prop) = expected_table_type.props.get(&key_str) {
                        if let Some(read_ty) = prop.read_ty {
                            *(*self.module)
                                .ast_expected_types
                                .get_or_insert(item.value as *const AstExpr) = read_ty;
                            is_subtype &=
                                self.test_potential_literal_is_subtype(item.value, read_ty);
                        }
                    } else if let Some(indexer) = &expected_table_type.indexer {
                        *(*self.module)
                            .ast_expected_types
                            .get_or_insert(item.key as *const AstExpr) = indexer.index_type;
                        *(*self.module)
                            .ast_expected_types
                            .get_or_insert(item.value as *const AstExpr) =
                            indexer.index_result_type;
                        let inferred_key_type = (*self.module).internal_types.add_type(
                            crate::records::singleton_type::SingletonType {
                                variant:
                                    crate::type_aliases::singleton_variant::SingletonVariant::V1(
                                        crate::records::string_singleton::StringSingleton {
                                            value: key_str.clone(),
                                        },
                                    ),
                            },
                        );
                        is_subtype &= self.test_is_subtype_type_id_type_id_location(
                            inferred_key_type,
                            indexer.index_type,
                            (*item.key).base.location,
                        );
                        is_subtype &= self.test_potential_literal_is_subtype(
                            item.value,
                            indexer.index_result_type,
                        );
                    }
                } else if item.kind == ItemKind::List {
                    if !is_array_like {
                        is_subtype = false;
                        self.report_error_type_error_data_location(
                            TypeErrorData::UnexpectedArrayLikeTableItem(
                                UnexpectedArrayLikeTableItem::default(),
                            ),
                            &(*item.value).base.location,
                        );
                    }

                    if let Some(indexer) = &expected_table_type.indexer {
                        *(*self.module)
                            .ast_expected_types
                            .get_or_insert(item.value as *const AstExpr) =
                            indexer.index_result_type;
                        is_subtype &= self.test_potential_literal_is_subtype(
                            item.value,
                            indexer.index_result_type,
                        );
                    }
                } else if item.kind == ItemKind::General {
                    if let Some(indexer) = &expected_table_type.indexer {
                        *(*self.module)
                            .ast_expected_types
                            .get_or_insert(item.key as *const AstExpr) = indexer.index_type;
                        *(*self.module)
                            .ast_expected_types
                            .get_or_insert(item.value as *const AstExpr) =
                            indexer.index_result_type;
                        is_subtype &=
                            self.test_potential_literal_is_subtype(item.key, indexer.index_type);
                        is_subtype &= self.test_potential_literal_is_subtype(
                            item.value,
                            indexer.index_result_type,
                        );
                    }
                }
            }

            if !missing_keys.is_empty() {
                self.report_error_type_error_data_location(
                    TypeErrorData::MissingProperties(MissingProperties {
                        super_type: expected_type,
                        sub_type: expr_type,
                        properties: missing_keys,
                        context: MissingPropertiesContext::Missing,
                    }),
                    &(*expr).base.location,
                );
                return false;
            }

            is_subtype
        }
    }
}
