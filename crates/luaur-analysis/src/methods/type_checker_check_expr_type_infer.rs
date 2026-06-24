use crate::enums::table_state::TableState;
use crate::enums::value_context::ValueContext;
use crate::functions::first::first;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::maybe_singleton::maybe_singleton;
use crate::functions::maybe_string::maybe_string;
use crate::functions::try_get_l_value::try_get_l_value;
use crate::records::count_mismatch::CountMismatchContext;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_error::GenericError;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::module::Module;
use crate::records::recursion_counter::RecursionCounter;
use crate::records::table_type::TableType;
use crate::records::truthy_predicate::TruthyPredicate;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::records::union_type::UnionType;
use crate::records::unknown_symbol::{Context, UnknownSymbol};
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::predicate::Predicate;
use crate::type_aliases::predicate_vec::PredicateVec;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;
use luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_error::AstExprError;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_instantiate::AstExprInstantiate;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;
use luaur_ast::records::ast_expr_unary::AstExprUnary;
use luaur_ast::records::ast_expr_varargs::AstExprVarargs;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::{ast_node_as, ast_node_is};
use luaur_common::FInt;

impl TypeChecker {
    pub fn check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExpr,
        expected_type: Option<TypeId>,
        force_singleton: bool,
    ) -> WithPredicate<TypeId> {
        let _rc = RecursionCounter::recursion_counter_i32(&mut self.check_recursion_count);
        let limit = FInt::LuauCheckRecursionLimit.get();
        if limit > 0 && self.check_recursion_count >= limit {
            self.report_error_code_too_complex(&expr.base.location);
            return WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope));
        }

        let node = expr as *const AstExpr as *mut AstNode;

        let mut result = if let Some(group) = unsafe { ast_node_as::<AstExprGroup>(node).as_ref() }
        {
            self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*group.expr },
                expected_type,
                false,
            )
        } else if ast_node_is::<AstExprConstantNil>(expr) {
            WithPredicate::with_predicate_t(self.nil_type)
        } else if let Some(bool_expr) = unsafe { ast_node_as::<AstExprConstantBool>(node).as_ref() }
        {
            let use_singleton = force_singleton || expected_type.map_or(false, maybe_singleton);
            WithPredicate::with_predicate_t(if use_singleton {
                self.singleton_type_bool(bool_expr.value)
            } else {
                self.boolean_type
            })
        } else if let Some(string_expr) =
            unsafe { ast_node_as::<AstExprConstantString>(node).as_ref() }
        {
            let use_singleton = force_singleton || expected_type.map_or(false, maybe_singleton);
            if use_singleton {
                let bytes = if string_expr.value.data.is_null() {
                    &[][..]
                } else {
                    unsafe {
                        core::slice::from_raw_parts(
                            string_expr.value.data as *const u8,
                            string_expr.value.size,
                        )
                    }
                };
                WithPredicate::with_predicate_t(self.singleton_type_string(
                    alloc::string::String::from_utf8_lossy(bytes).into_owned(),
                ))
            } else {
                WithPredicate::with_predicate_t(self.string_type)
            }
        } else if ast_node_is::<AstExprConstantNumber>(expr) {
            WithPredicate::with_predicate_t(self.number_type)
        } else if ast_node_is::<AstExprConstantInteger>(expr) {
            WithPredicate::with_predicate_t(self.integer_type)
        } else if let Some(local_expr) = unsafe { ast_node_as::<AstExprLocal>(node).as_ref() } {
            let lvalue = try_get_l_value(&local_expr.base);
            if let Some(lvalue) = lvalue {
                if let Some(ty) = self.resolve_l_value_scope_ptr_l_value(scope.clone(), &lvalue) {
                    WithPredicate::with_predicate_t_predicate_vec(
                        ty,
                        PredicateVec::from(alloc::vec![Predicate::Truthy(TruthyPredicate {
                            lvalue,
                            location: local_expr.base.base.location,
                        })]),
                    )
                } else {
                    let name = unsafe {
                        if (*local_expr.local).name.value.is_null() {
                            alloc::string::String::new()
                        } else {
                            core::ffi::CStr::from_ptr((*local_expr.local).name.value)
                                .to_string_lossy()
                                .into_owned()
                        }
                    };
                    self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                        local_expr.base.base.location,
                        TypeErrorData::UnknownSymbol(UnknownSymbol::new(name, Context::Binding)),
                    ));
                    WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
                }
            } else {
                self.ice_string_location(
                    "AstExprLocal exists but no LValue was produced",
                    &expr.base.location,
                );
                WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
            }
        } else if let Some(global_expr) = unsafe { ast_node_as::<AstExprGlobal>(node).as_ref() } {
            self.check_expr_scope_ptr_ast_expr_global(scope, global_expr)
        } else if let Some(varargs_expr) = unsafe { ast_node_as::<AstExprVarargs>(node).as_ref() } {
            let vararg_pack = unsafe {
                follow_type_pack_id(self.check_expr_pack(scope, &varargs_expr.base).r#type)
            };

            if unsafe { !get_type_pack_id::<TypePack>(vararg_pack).is_null() } {
                WithPredicate::with_predicate_t(first(vararg_pack, false).unwrap_or(self.nil_type))
            } else if unsafe { !get_type_pack_id::<FreeTypePack>(vararg_pack).is_null() } {
                let head = self.fresh_type_scope_ptr(scope.clone());
                let tail = self.fresh_type_pack_scope_ptr(scope.clone());
                let pack = unsafe { get_mutable_type_pack_id::<TypePack>(vararg_pack) };
                if !pack.is_null() {
                    unsafe {
                        *pack = TypePack {
                            head: alloc::vec![head],
                            tail: Some(tail),
                        };
                    }
                } else {
                    unsafe {
                        *crate::functions::as_mutable_type_pack::as_mutable_type_pack_id(
                            vararg_pack,
                        ) = TypePackVar::from(TypePack {
                            head: alloc::vec![head],
                            tail: Some(tail),
                        });
                    }
                }
                WithPredicate::with_predicate_t(head)
            } else if unsafe { !get_type_pack_id::<ErrorTypePack>(vararg_pack).is_null() } {
                WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
            } else if let Some(vtp) =
                unsafe { get_type_pack_id::<VariadicTypePack>(vararg_pack).as_ref() }
            {
                WithPredicate::with_predicate_t(vtp.ty)
            } else if unsafe { !get_type_pack_id::<GenericTypePack>(vararg_pack).is_null() } {
                self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                    varargs_expr.base.base.location,
                    TypeErrorData::GenericError(GenericError::new(alloc::string::String::from(
                        "Trying to get a type from a variadic type parameter",
                    ))),
                ));
                WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
            } else {
                self.ice_string_location(
                    "Unknown TypePack type in checkExpr(AstExprVarargs)",
                    &expr.base.location,
                );
                WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
            }
        } else if let Some(call_expr) = unsafe { ast_node_as::<AstExprCall>(node).as_ref() } {
            let pack_result = self.check_expr_pack(scope, &call_expr.base);
            let ret_pack = unsafe { follow_type_pack_id(pack_result.r#type) };

            if let Some(pack) = unsafe { get_type_pack_id::<TypePack>(ret_pack).as_ref() } {
                WithPredicate::with_predicate_t_predicate_vec(
                    pack.head.first().copied().unwrap_or(self.nil_type),
                    pack_result.predicates,
                )
            } else if unsafe { !get_type_pack_id::<FreeTypePack>(ret_pack).is_null() } {
                let head = self.fresh_type_type_level(scope.level);
                let tail = self.fresh_type_pack_type_level(scope.level);
                let pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
                    head: alloc::vec![head],
                    tail: Some(tail),
                }));
                self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
                    pack,
                    ret_pack,
                    scope,
                    &call_expr.base.base.location,
                    CountMismatchContext::Arg,
                );
                WithPredicate::with_predicate_t_predicate_vec(head, pack_result.predicates)
            } else if unsafe { !get_type_pack_id::<ErrorTypePack>(ret_pack).is_null() } {
                WithPredicate::with_predicate_t_predicate_vec(
                    self.error_recovery_type_scope_ptr(scope),
                    pack_result.predicates,
                )
            } else if let Some(vtp) =
                unsafe { get_type_pack_id::<VariadicTypePack>(ret_pack).as_ref() }
            {
                WithPredicate::with_predicate_t_predicate_vec(vtp.ty, pack_result.predicates)
            } else if unsafe { !get_type_pack_id::<GenericTypePack>(ret_pack).is_null() } {
                WithPredicate::with_predicate_t_predicate_vec(self.any_type, pack_result.predicates)
            } else {
                self.ice_string_location(
                    "Unknown TypePack type in checkExpr(AstExprCall)",
                    &expr.base.location,
                );
                WithPredicate::with_predicate_t_predicate_vec(
                    self.error_recovery_type_scope_ptr(scope),
                    pack_result.predicates,
                )
            }
        } else if let Some(index_name) = unsafe { ast_node_as::<AstExprIndexName>(node).as_ref() } {
            self.check_expr_scope_ptr_ast_expr_index_name(scope, index_name)
        } else if let Some(index_expr) = unsafe { ast_node_as::<AstExprIndexExpr>(node).as_ref() } {
            let ty = self.check_l_value(scope, &index_expr.base, ValueContext::RValue);
            if let Some(lvalue) = try_get_l_value(&index_expr.base) {
                if let Some(refined_ty) =
                    self.resolve_l_value_scope_ptr_l_value(scope.clone(), &lvalue)
                {
                    WithPredicate::with_predicate_t_predicate_vec(
                        refined_ty,
                        PredicateVec::from(alloc::vec![Predicate::Truthy(TruthyPredicate {
                            lvalue,
                            location: index_expr.base.base.location,
                        })]),
                    )
                } else {
                    WithPredicate::with_predicate_t(ty)
                }
            } else {
                WithPredicate::with_predicate_t(ty)
            }
        } else if let Some(function_expr) = unsafe { ast_node_as::<AstExprFunction>(node).as_ref() }
        {
            self.check_expr_scope_ptr_ast_expr_function_optional_type_id(
                scope,
                function_expr,
                expected_type,
            )
        } else if let Some(table_expr) = unsafe { ast_node_as::<AstExprTable>(node).as_ref() } {
            let _table_rc =
                RecursionCounter::recursion_counter_i32(&mut self.check_recursion_count);
            if limit > 0 && self.check_recursion_count >= limit {
                self.report_error_code_too_complex(&table_expr.base.base.location);
                return WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope));
            }

            let mut field_types = alloc::vec::Vec::new();
            field_types.reserve(table_expr.items.size);

            let mut expected_table: Option<&TableType> = None;
            let mut expected_union: Option<&UnionType> = None;
            let mut expected_index_type: Option<TypeId> = None;
            let mut expected_index_result_type: Option<TypeId> = None;

            if let Some(expected_type) = expected_type {
                let followed = unsafe { follow_type_id(expected_type) };
                let ttv = unsafe { get_type_id::<TableType>(followed) };
                if !ttv.is_null() {
                    let ttv = unsafe { &*ttv };
                    if ttv.state == TableState::Sealed {
                        expected_table = Some(ttv);

                        if let Some(indexer) = &ttv.indexer {
                            expected_index_type = Some(indexer.index_type);
                            expected_index_result_type = Some(indexer.index_result_type);
                        }
                    }
                } else {
                    let utv = unsafe { get_type_id::<UnionType>(followed) };
                    if !utv.is_null() {
                        expected_union = Some(unsafe { &*utv });
                    }
                }
            }

            for item in table_expr.items.iter() {
                let mut expected_result_type: Option<TypeId> = None;
                let mut is_indexed_item = false;

                if item.kind == luaur_ast::records::ast_expr_table::ItemKind::List {
                    expected_result_type = expected_index_result_type;
                    is_indexed_item = true;
                } else if item.kind == luaur_ast::records::ast_expr_table::ItemKind::Record
                    || item.kind == luaur_ast::records::ast_expr_table::ItemKind::General
                {
                    if !item.key.is_null() {
                        let key = unsafe {
                            ast_node_as::<AstExprConstantString>(item.key as *mut AstNode)
                        };
                        if !key.is_null() {
                            let key_str = {
                                let bytes = unsafe {
                                    core::slice::from_raw_parts(
                                        (*key).value.data as *const u8,
                                        (*key).value.size,
                                    )
                                };
                                alloc::string::String::from(
                                    core::str::from_utf8(bytes).unwrap_or(""),
                                )
                            };

                            if let Some(expected_table) = expected_table {
                                if let Some(prop) = expected_table.props.get(&key_str) {
                                    expected_result_type = Some(prop.type_deprecated());
                                } else if expected_index_type.map_or(false, maybe_string) {
                                    expected_result_type = expected_index_result_type;
                                }
                            } else if let Some(expected_union) = expected_union {
                                let mut expected_result_types = alloc::vec::Vec::new();

                                for &expected_option in &expected_union.options {
                                    let ttv = unsafe {
                                        get_type_id::<TableType>(follow_type_id(expected_option))
                                    };
                                    if ttv.is_null() {
                                        continue;
                                    }

                                    let ttv = unsafe { &*ttv };
                                    if let Some(prop) = ttv.props.get(&key_str) {
                                        expected_result_types.push(prop.type_deprecated());
                                    } else if let Some(indexer) = &ttv.indexer {
                                        if maybe_string(indexer.index_type) {
                                            expected_result_types.push(indexer.index_result_type);
                                        }
                                    }
                                }

                                if expected_result_types.len() == 1 {
                                    expected_result_type = Some(expected_result_types[0]);
                                } else if expected_result_types.len() > 1 {
                                    expected_result_type = Some(self.add_type(&UnionType {
                                        options: expected_result_types,
                                    }));
                                }
                            }
                        } else {
                            expected_result_type = expected_index_result_type;
                            is_indexed_item = true;
                        }
                    } else {
                        expected_result_type = expected_index_result_type;
                        is_indexed_item = true;
                    }
                }

                let key_type = if item.key.is_null() {
                    self.number_type
                } else {
                    self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                        scope,
                        unsafe { &*item.key },
                        expected_index_type,
                        false,
                    )
                    .r#type
                };
                let value_type = self
                    .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                        scope,
                        unsafe { &*item.value },
                        expected_result_type,
                        false,
                    )
                    .r#type;
                field_types.push((key_type, value_type));

                if is_indexed_item && expected_index_result_type.is_none() {
                    expected_index_result_type = Some(value_type);
                }
            }
            WithPredicate::with_predicate_t(self.check_expr_table(
                scope,
                table_expr,
                &field_types,
                expected_type,
            ))
        } else if let Some(unary_expr) = unsafe { ast_node_as::<AstExprUnary>(node).as_ref() } {
            self.check_expr_scope_ptr_ast_expr_unary(scope, unary_expr)
        } else if let Some(binary_expr) = unsafe { ast_node_as::<AstExprBinary>(node).as_ref() } {
            self.check_expr_scope_ptr_ast_expr_binary_optional_type_id(
                scope,
                binary_expr,
                expected_type,
            )
        } else if let Some(type_assertion) =
            unsafe { ast_node_as::<AstExprTypeAssertion>(node).as_ref() }
        {
            self.check_expr_scope_ptr_ast_expr_type_assertion(scope, type_assertion)
        } else if let Some(error_expr) = unsafe { ast_node_as::<AstExprError>(node).as_ref() } {
            let old_size = unsafe {
                let module =
                    alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
                (*module).errors.len()
            };
            for child in error_expr.expressions.iter() {
                self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                    scope,
                    unsafe { &**child },
                    None,
                    false,
                );
            }
            unsafe {
                let module =
                    alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
                (*module).errors.truncate(old_size);
            }
            WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
        } else if let Some(if_else) = unsafe { ast_node_as::<AstExprIfElse>(node).as_ref() } {
            self.check_expr_scope_ptr_ast_expr_if_else_optional_type_id(
                scope,
                if_else,
                expected_type,
            )
        } else if let Some(interp) = unsafe { ast_node_as::<AstExprInterpString>(node).as_ref() } {
            self.check_expr_scope_ptr_ast_expr_interp_string(scope, interp)
        } else if let Some(instantiate) =
            unsafe { ast_node_as::<AstExprInstantiate>(node).as_ref() }
        {
            self.check_expr_scope_ptr_ast_expr_instantiate(scope, instantiate)
        } else {
            self.ice_string_location("Unhandled AstExpr", &expr.base.location);
            WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
        };

        result.r#type = unsafe { follow_type_id(result.r#type) };

        unsafe {
            let module =
                alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
            let key = expr as *const AstExpr;
            if (*module).ast_types.find(&key).is_none() {
                *(*module).ast_types.get_or_insert(key) = result.r#type;
            }
            if let Some(expected_type) = expected_type {
                *(*module).ast_expected_types.get_or_insert(key) = expected_type;
            }
        }

        result
    }
}
