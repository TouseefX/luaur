//! Faithful port of `BidirectionalTypePusher::pushType`
//! (`Analysis/src/TableLiteralInference.cpp:115-359`).

use alloc::vec::Vec;

use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;
use luaur_ast::records::ast_expr_table::{AstExprTable, ItemKind};
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as_const;

use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

use crate::enums::relation::Relation;
use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::contains_generic_type_utils::contains_generic_type_id_not_null_dense_hash_set_void;
use crate::functions::contains_generic_type_utils_alt_b::contains_generic;
use crate::functions::extract_matching_table_type::extract_matching_table_type;
use crate::functions::extract_matching_table_type_deprecated::extract_matching_table_type_deprecated;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::is_literal::is_literal;
use crate::functions::is_record::is_record;
use crate::functions::maybe_singleton::maybe_singleton;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::functions::strip_nil::strip_nil;
use crate::records::any_type::AnyType;
use crate::records::bidirectional_type_pusher::BidirectionalTypePusher;
use crate::records::blocked_type::BlockedType;
use crate::records::find_function_type_in::FindFunctionTypeIn;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::function_type::FunctionType;
use crate::records::incomplete_inference::IncompleteInference;
use crate::records::intersection_type::IntersectionType;
use crate::records::iterative_type_visitor::IterativeTypeVisitorTrait;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

impl BidirectionalTypePusher {
    pub fn push_type(&mut self, expected_type: TypeId, expr: *const AstExpr) -> TypeId {
        let mut expected_type = expected_type;
        unsafe {
            *(*self.astExpectedTypes).get_or_insert(expr) = expected_type;
            if !(*self.astTypes).contains(&expr) {
                return (*(*self.solver).builtin_types).anyType;
            }

            let mut expr_type = *(*self.astTypes).find(&expr).unwrap();

            if self.seen.contains(&(expected_type, expr)) {
                return expr_type;
            }
            self.seen.insert((expected_type, expr));

            expected_type = follow_type_id(expected_type);
            expr_type = follow_type_id(expr_type);

            let tfit = get_type_id::<TypeFunctionInstanceType>(expected_type);
            if !tfit.is_null() && (*tfit).state == TypeFunctionInstanceState::Unsolved {
                self.incompleteInferences.push(IncompleteInference {
                    expectedType: expected_type,
                    targetType: expr_type,
                    expr,
                });
                return expr_type;
            }

            if !get_type_id::<BlockedType>(expected_type).is_null()
                || !get_type_id::<PendingExpansionType>(expected_type).is_null()
            {
                self.incompleteInferences.push(IncompleteInference {
                    expectedType: expected_type,
                    targetType: expr_type,
                    expr,
                });
                return expr_type;
            }

            if !get_type_id::<AnyType>(expected_type).is_null()
                || !get_type_id::<UnknownType>(expected_type).is_null()
            {
                return expr_type;
            }

            let group = ast_node_as_const::<AstExprGroup>(expr as *const AstNode);
            if !group.is_null() {
                self.push_type(expected_type, (*group).expr as *const AstExpr);
                return expr_type;
            }

            let ternary = ast_node_as_const::<AstExprIfElse>(expr as *const AstNode);
            if !ternary.is_null() {
                self.push_type(expected_type, (*ternary).true_expr as *const AstExpr);
                self.push_type(expected_type, (*ternary).false_expr as *const AstExpr);
                return expr_type;
            }

            if !is_literal(expr) {
                return expr_type;
            }

            if !ast_node_as_const::<AstExprConstantString>(expr as *const AstNode).is_null()
                || !ast_node_as_const::<AstExprConstantNumber>(expr as *const AstNode).is_null()
                || !ast_node_as_const::<AstExprConstantBool>(expr as *const AstNode).is_null()
                || !ast_node_as_const::<AstExprConstantNil>(expr as *const AstNode).is_null()
            {
                let ft = get_type_id::<FreeType>(expr_type);
                if !ft.is_null() {
                    if maybe_singleton(expected_type) && maybe_singleton((*ft).lower_bound) {
                        (*self.solver).bind_not_null_constraint_type_id_type_id(
                            self.constraint,
                            expr_type,
                            (*ft).lower_bound,
                        );
                        return expr_type;
                    }

                    let upper_bound_relation =
                        relate_type_id_type_id((*ft).upper_bound, expected_type);
                    if upper_bound_relation == Relation::Subset
                        || upper_bound_relation == Relation::Coincident
                    {
                        (*self.solver).bind_not_null_constraint_type_id_type_id(
                            self.constraint,
                            expr_type,
                            expected_type,
                        );
                        return expr_type;
                    }

                    let lower_bound_relation =
                        relate_type_id_type_id((*ft).lower_bound, expected_type);
                    if lower_bound_relation == Relation::Subset
                        || lower_bound_relation == Relation::Coincident
                    {
                        (*self.solver).bind_not_null_constraint_type_id_type_id(
                            self.constraint,
                            expr_type,
                            expected_type,
                        );
                        return expr_type;
                    }
                }
            }

            let expr_lambda = ast_node_as_const::<AstExprFunction>(expr as *const AstNode);
            if !expr_lambda.is_null() {
                let lambda_ty = get_type_id::<FunctionType>(expr_type);
                let expected_lambda_ty: *const FunctionType;
                if FFlag::LuauBidirectionalInferenceBetterUnionHandling.get() {
                    let mut ffti =
                        FindFunctionTypeIn::find_function_type_in((*expr_lambda).args.size as i32);
                    ffti.run_type_id(expected_type);
                    expected_lambda_ty = ffti.candidate;
                } else {
                    expected_lambda_ty = get_type_id::<FunctionType>(strip_nil(
                        (*self.solver).builtin_types,
                        &mut *(*self.solver).arena,
                        expected_type,
                    ));
                }

                if !lambda_ty.is_null() && !expected_lambda_ty.is_null() {
                    let (lambda_arg_tys, _lambda_tail) =
                        flatten_type_pack_id((*lambda_ty).arg_types);
                    let (expected_lambda_arg_tys, _expected_lambda_tail) =
                        flatten_type_pack_id((*expected_lambda_ty).arg_types);
                    let limit = lambda_arg_tys
                        .len()
                        .min(expected_lambda_arg_tys.len())
                        .min((*expr_lambda).args.size);
                    for arg_index in 0..limit {
                        let local = *(*expr_lambda).args.data.add(arg_index);
                        if (*local).annotation.is_null()
                            && !get_type_id::<FreeType>(follow_type_id(lambda_arg_tys[arg_index]))
                                .is_null()
                            && !contains_generic_type_id_not_null_dense_hash_set_void(
                                expected_lambda_arg_tys[arg_index],
                                self.genericTypesAndPacks,
                            )
                        {
                            (*self.solver).bind_not_null_constraint_type_id_type_id(
                                self.constraint,
                                lambda_arg_tys[arg_index],
                                expected_lambda_arg_tys[arg_index],
                            );
                        }
                    }

                    if (*expr_lambda).return_annotation.is_null()
                        && !get_type_pack_id::<FreeTypePack>(follow_type_pack_id(
                            (*lambda_ty).ret_types,
                        ))
                        .is_null()
                        && !contains_generic(
                            (*expected_lambda_ty).ret_types,
                            self.genericTypesAndPacks,
                        )
                    {
                        (*self.solver).bind_not_null_constraint_type_pack_id_type_pack_id(
                            self.constraint,
                            (*lambda_ty).ret_types,
                            (*expected_lambda_ty).ret_types,
                        );
                    }
                }
            }

            let expr_table = ast_node_as_const::<AstExprTable>(expr as *const AstNode);
            if !expr_table.is_null() {
                let expected_table_ty = get_type_id::<TableType>(expected_type);
                if expected_table_ty.is_null() {
                    let utv = get_type_id::<UnionType>(expected_type);
                    if !utv.is_null() {
                        if FFlag::LuauBidirectionalInferenceBetterUnionHandling.get() {
                            if let Some(tt) = extract_matching_table_type(
                                &*utv,
                                expr_type,
                                (*self.solver).builtin_types,
                            ) {
                                let _ = self.push_type(tt, expr);
                            }
                        } else {
                            let mut parts: Vec<TypeId> = (*utv).options.clone();
                            if let Some(tt) = extract_matching_table_type_deprecated(
                                &mut parts,
                                expr_type,
                                (*self.solver).builtin_types,
                            ) {
                                let _ = self.push_type(tt, expr);
                            }
                        }
                    } else {
                        let itv = get_type_id::<IntersectionType>(expected_type);
                        if !itv.is_null() {
                            let parts: Vec<TypeId> = (*itv).parts.clone();
                            for part in parts {
                                let _ = self.push_type(part, expr);
                            }
                            *(*self.astExpectedTypes).get_or_insert(expr) = expected_type;
                        }
                    }
                    return expr_type;
                }

                for idx in 0..(*expr_table).items.size {
                    let item = (*expr_table).items.data.add(idx);
                    if is_record(&*item) {
                        let key_const_string = ast_node_as_const::<AstExprConstantString>(
                            (*item).key as *const AstNode,
                        );
                        let s = &(*key_const_string).value;
                        let slice =
                            core::slice::from_raw_parts(s.data as *const u8, s.size as usize);
                        let key_str = core::str::from_utf8_unchecked(slice).to_string();

                        let read_ty_opt =
                            (*expected_table_ty).props.get(&key_str).map(|p| p.read_ty);
                        match read_ty_opt {
                            None => {
                                let index_result_type = (*expected_table_ty)
                                    .indexer
                                    .as_ref()
                                    .map(|ix| ix.index_result_type);
                                if let Some(index_result_type) = index_result_type {
                                    let _ = self.push_type(
                                        index_result_type,
                                        (*item).value as *const AstExpr,
                                    );
                                }
                                continue;
                            }
                            Some(read_ty) => {
                                if let Some(read_ty) = read_ty {
                                    let _ =
                                        self.push_type(read_ty, (*item).value as *const AstExpr);
                                }
                            }
                        }
                    } else if (*item).kind == ItemKind::List {
                        let index_pair = (*expected_table_ty)
                            .indexer
                            .as_ref()
                            .map(|ix| (ix.index_type, ix.index_result_type));
                        if let Some((index_type, index_result_type)) = index_pair {
                            (*self.unifier)
                                .unify(index_type, (*(*self.solver).builtin_types).numberType);
                            let _ =
                                self.push_type(index_result_type, (*item).value as *const AstExpr);
                        }
                    } else if (*item).kind == ItemKind::General {
                        let index_pair = (*expected_table_ty)
                            .indexer
                            .as_ref()
                            .map(|ix| (ix.index_type, ix.index_result_type));
                        if let Some((index_type, index_result_type)) = index_pair {
                            let _ = self.push_type(index_type, (*item).key as *const AstExpr);
                            let _ =
                                self.push_type(index_result_type, (*item).value as *const AstExpr);
                        }
                    } else {
                        LUAU_ASSERT!(false /* "Unexpected" */);
                    }
                }
            }

            expr_type
        }
    }
}
