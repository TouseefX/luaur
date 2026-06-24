//! Source: `Analysis/src/TypeInfer.cpp` (TypeChecker::checkRelationalOperation, L2713-3024)
use crate::enums::op_kind::OpKind;
use crate::functions::are_eq_comparable::are_eq_comparable;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_identifier_of_base_var_type_infer::get_identifier_of_base_var;
use crate::functions::get_metatable_type::get_metatable_type_id_not_null_builtin_types;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_boolean::is_boolean;
use crate::functions::is_nil::is_nil;
use crate::functions::is_prim::is_prim;
use crate::functions::is_string::is_string;
use crate::functions::op_to_meta_table_entry::op_to_meta_table_entry;
use crate::records::any_type::AnyType;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::cannot_infer_binary_operation::CannotInferBinaryOperation;
use crate::records::error_type::ErrorType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::type_checker::TypeChecker;
use crate::records::union_type::UnionType;
use crate::type_aliases::predicate_vec::PredicateVec;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::singleton_variant::SingletonVariantMember;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::format;
use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinary_Op};
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl TypeChecker {
    pub fn check_relational_operation(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprBinary,
        lhs_type: TypeId,
        rhs_type: TypeId,
        predicates: &PredicateVec,
    ) -> TypeId {
        let is_equality =
            expr.op == AstExprBinary_Op::CompareEq || expr.op == AstExprBinary_Op::CompareNe;

        let lhs_type = self.relational_strip_nil(lhs_type, expr.op == AstExprBinary_Op::Or);
        let rhs_type = self.relational_strip_nil(rhs_type, false);

        // If we know nothing at all about the lhs type, we can usually say nothing about the result.
        // The notable exception to this is the equality and inequality operators, which always produce a boolean.
        let lhs_is_any = unsafe {
            !get_type_id::<AnyType>(lhs_type).is_null()
                || !get_type_id::<ErrorType>(lhs_type).is_null()
                || !get_type_id::<NeverType>(lhs_type).is_null()
        };

        // Peephole check for `cond and a or b -> type(a)|type(b)`
        // TODO: Kill this when singleton types arrive. :(
        let subexp = unsafe { ast_node_as::<AstExprBinary>(expr.left as *mut AstNode) };
        if !subexp.is_null() {
            if expr.op == AstExprBinary_Op::Or && unsafe { (*subexp).op } == AstExprBinary_Op::And {
                let sub_scope = self.child_scope(scope, &unsafe { (*subexp).base.base.location });
                self.resolve_predicate_vec_scope_ptr_bool(predicates, &sub_scope, true);
                let right_ty = self
                    .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                        &sub_scope,
                        unsafe { &*(*subexp).right },
                        None,
                        false,
                    )
                    .r#type;
                let stripped = self.relational_strip_nil(right_ty, true);
                return self.union_of_types(
                    rhs_type,
                    stripped,
                    &sub_scope,
                    &expr.base.base.location,
                    true,
                );
            }
        }

        // Lua casts the results of these to boolean
        match expr.op {
            AstExprBinary_Op::CompareNe
            | AstExprBinary_Op::CompareEq
            | AstExprBinary_Op::CompareLt
            | AstExprBinary_Op::CompareGt
            | AstExprBinary_Op::CompareGe
            | AstExprBinary_Op::CompareLe => {
                if expr.op == AstExprBinary_Op::CompareNe || expr.op == AstExprBinary_Op::CompareEq
                {
                    if self.is_nonstrict_mode() && (is_nil(lhs_type) || is_nil(rhs_type)) {
                        return self.boolean_type;
                    }

                    let rhs_is_any = unsafe {
                        !get_type_id::<AnyType>(rhs_type).is_null()
                            || !get_type_id::<ErrorType>(rhs_type).is_null()
                            || !get_type_id::<NeverType>(rhs_type).is_null()
                    };
                    if lhs_is_any || rhs_is_any {
                        return self.boolean_type;
                    }
                    // [[fallthrough]] into the comparison body below.
                }

                // If one of the operand is never, it doesn't make sense to unify these.
                if unsafe {
                    !get_type_id::<NeverType>(lhs_type).is_null()
                        || !get_type_id::<NeverType>(rhs_type).is_null()
                } {
                    return self.boolean_type;
                }

                if is_equality {
                    // Unless either type is free or any, an equality comparison is only
                    // valid when the intersection of the two operands is non-empty.
                    //
                    // eg it is okay to compare string? == number? because the two types
                    // have nil in common, but string == number is not allowed.
                    let arena_ptr: *mut crate::records::type_arena::TypeArena = unsafe {
                        &mut (*(alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                            as *mut crate::records::module::Module))
                            .internal_types
                    };
                    let normalizer_ptr: *mut crate::records::normalizer::Normalizer =
                        &mut self.normalizer;
                    let eq_test_result = are_eq_comparable(
                        unsafe { &mut *arena_ptr },
                        unsafe { &mut *normalizer_ptr },
                        lhs_type,
                        rhs_type,
                    );
                    if eq_test_result.is_none() {
                        self.report_error_code_too_complex(&expr.base.base.location);
                        return self.error_recovery_type_type_id(self.boolean_type);
                    }

                    if !eq_test_result.unwrap() {
                        self.report_error_location_type_error_data(
                            &expr.base.base.location,
                            TypeErrorData::GenericError(GenericError::new(format!(
                                "Type {} cannot be compared with {}",
                                crate::functions::to_string_to_string_alt_c::to_string_type_id(
                                    lhs_type
                                ),
                                crate::functions::to_string_to_string_alt_c::to_string_type_id(
                                    rhs_type
                                )
                            ))),
                        );
                        return self.error_recovery_type_type_id(self.boolean_type);
                    }
                }

                /* Subtlety here:
                 * We need to do this unification first, but there are situations where we don't actually want to
                 * report any problems that might have been surfaced as a result of this step because we might already
                 * have a better, more descriptive error teed up.
                 */
                let mut state = self.mk_unifier(scope, &expr.base.base.location);
                if !is_equality {
                    state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
                        rhs_type, lhs_type, false, false, None,
                    );
                    state.log.commit();
                }

                let needs_metamethod = !is_equality;

                let left_type = unsafe { follow_type_id(lhs_type) };
                if unsafe {
                    !get_type_id::<PrimitiveType>(left_type).is_null()
                        || !get_type_id::<AnyType>(left_type).is_null()
                        || !get_type_id::<ErrorType>(left_type).is_null()
                        || !get_type_id::<UnionType>(left_type).is_null()
                } {
                    self.report_errors(&state.errors);

                    // The original version of this check also produced this error when we had a union type.
                    // However, the old solver does not readily have the ability to discern if the union is comparable.
                    // This is the case when the lhs is e.g. a union of singletons and the rhs is the combined type.
                    // The new solver has much more powerful logic for resolving relational operators, but for now,
                    // we need to be conservative in the old solver to deliver a reasonable developer experience.
                    if !is_equality && state.errors.is_empty() && is_boolean(left_type) {
                        self.report_error_location_type_error_data(
                            &expr.base.base.location,
                            TypeErrorData::GenericError(GenericError::new(format!(
                                "Type '{}' cannot be compared with relational operator {}",
                                crate::functions::to_string_to_string_alt_c::to_string_type_id(
                                    left_type
                                ),
                                luaur_ast::functions::to_string_ast_alt_b::to_string(expr.op)
                            ))),
                        );
                    }

                    return self.boolean_type;
                }

                let metamethod_name = op_to_meta_table_entry(expr.op);

                let string_no_mt: Option<TypeId> = None; // works around gcc false positive "maybe uninitialized" warnings
                let left_metatable: Option<TypeId> = if is_string(lhs_type) {
                    string_no_mt
                } else {
                    get_metatable_type_id_not_null_builtin_types(
                        unsafe { follow_type_id(lhs_type) },
                        unsafe { &*self.builtin_types },
                    )
                };
                let right_metatable: Option<TypeId> = if is_string(rhs_type) {
                    string_no_mt
                } else {
                    get_metatable_type_id_not_null_builtin_types(
                        unsafe { follow_type_id(rhs_type) },
                        unsafe { &*self.builtin_types },
                    )
                };

                if left_metatable != right_metatable {
                    let mut matches = false;
                    if is_equality {
                        let utv = unsafe { get_type_id::<UnionType>(left_type) };
                        if !utv.is_null() && right_metatable.is_some() {
                            for &left_option in unsafe { (*utv).options.iter() } {
                                if get_metatable_type_id_not_null_builtin_types(
                                    unsafe { follow_type_id(left_option) },
                                    unsafe { &*self.builtin_types },
                                ) == right_metatable
                                {
                                    matches = true;
                                    break;
                                }
                            }
                        }

                        if !matches {
                            let utv = unsafe { get_type_id::<UnionType>(rhs_type) };
                            if !utv.is_null() && left_metatable.is_some() {
                                for &right_option in unsafe { (*utv).options.iter() } {
                                    if get_metatable_type_id_not_null_builtin_types(
                                        unsafe { follow_type_id(right_option) },
                                        unsafe { &*self.builtin_types },
                                    ) == left_metatable
                                    {
                                        matches = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if !matches {
                        self.report_error_location_type_error_data(
                            &expr.base.base.location,
                            TypeErrorData::GenericError(GenericError::new(format!(
                                "Types {} and {} cannot be compared with {} because they do not have the same metatable",
                                crate::functions::to_string_to_string_alt_c::to_string_type_id(lhs_type),
                                crate::functions::to_string_to_string_alt_c::to_string_type_id(rhs_type),
                                luaur_ast::functions::to_string_ast_alt_b::to_string(expr.op)
                            ))),
                        );
                        return self.error_recovery_type_type_id(self.boolean_type);
                    }
                }

                if left_metatable.is_some() {
                    let metamethod = self.find_metatable_entry(
                        lhs_type,
                        metamethod_name.clone(),
                        &expr.base.base.location,
                        true,
                    );
                    if let Some(metamethod) = metamethod {
                        let ftv =
                            unsafe { get_type_id::<FunctionType>(follow_type_id(metamethod)) };
                        if !ftv.is_null() {
                            if is_equality {
                                let bool_pack = self
                                    .add_type_pack_initializer_list_type_id(&[self.boolean_type]);
                                let ret_types = unsafe { (*ftv).ret_types };
                                state.try_unify_type_pack_id_type_pack_id_bool_entry(
                                    bool_pack, ret_types, false,
                                );

                                if !state.errors.is_empty() {
                                    self.report_error_location_type_error_data(
                                        &expr.base.base.location,
                                        TypeErrorData::GenericError(GenericError::new(format!(
                                            "Metamethod '{}' must return type 'boolean'",
                                            metamethod_name
                                        ))),
                                    );
                                    return self.error_recovery_type_type_id(self.boolean_type);
                                }

                                state.log.commit();
                            }
                        }

                        self.report_errors(&state.errors);

                        let arg_pack =
                            self.add_type_pack_initializer_list_type_id(&[lhs_type, rhs_type]);
                        let ret_pack =
                            self.add_type_pack_initializer_list_type_id(&[self.boolean_type]);
                        let mut ftv2 =
                            FunctionType::function_type_new(arg_pack, ret_pack, None, false);
                        ftv2.level = scope.level;
                        let actual_function_type = self.add_type_tv_internal(ftv2);
                        let inst_actual = self.instantiate(
                            scope,
                            actual_function_type,
                            expr.base.base.location,
                            core::ptr::null(),
                        );
                        let inst_meta = self.instantiate(
                            scope,
                            metamethod,
                            expr.base.base.location,
                            core::ptr::null(),
                        );
                        state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
                            inst_actual,
                            inst_meta,
                            true,
                            false,
                            None,
                        );

                        state.log.commit();

                        self.report_errors(&state.errors);
                        return self.boolean_type;
                    } else if needs_metamethod {
                        self.report_error_location_type_error_data(
                            &expr.base.base.location,
                            TypeErrorData::GenericError(GenericError::new(format!(
                                "Table {} does not offer metamethod {}",
                                crate::functions::to_string_to_string_alt_c::to_string_type_id(
                                    lhs_type
                                ),
                                metamethod_name
                            ))),
                        );
                        return self.error_recovery_type_type_id(self.boolean_type);
                    }
                }

                if unsafe { !get_type_id::<FreeType>(follow_type_id(lhs_type)).is_null() }
                    && !is_equality
                {
                    let name = get_identifier_of_base_var(expr.left);
                    self.report_error_location_type_error_data(
                        &expr.base.base.location,
                        TypeErrorData::CannotInferBinaryOperation(CannotInferBinaryOperation::new(
                            expr.op,
                            name,
                            OpKind::Comparison,
                        )),
                    );
                    return self.error_recovery_type_type_id(self.boolean_type);
                }

                if needs_metamethod {
                    self.report_error_location_type_error_data(
                        &expr.base.base.location,
                        TypeErrorData::GenericError(GenericError::new(format!(
                            "Type {} cannot be compared with {} because it has no metatable",
                            crate::functions::to_string_to_string_alt_c::to_string_type_id(
                                lhs_type
                            ),
                            luaur_ast::functions::to_string_ast_alt_b::to_string(expr.op)
                        ))),
                    );
                    return self.error_recovery_type_type_id(self.boolean_type);
                }

                self.boolean_type
            }

            AstExprBinary_Op::And => {
                if lhs_is_any {
                    lhs_type
                } else {
                    // If lhs is free, we can't tell which 'falsy' components it has, if any
                    if unsafe { !get_type_id::<FreeType>(lhs_type).is_null() } {
                        let false_singleton = self.singleton_type_bool(false);
                        let union_ty = self.add_type_tv_internal(UnionType {
                            options: alloc::vec![self.nil_type, false_singleton],
                        });
                        return self.union_of_types(
                            union_ty,
                            rhs_type,
                            scope,
                            &expr.base.base.location,
                            false,
                        );
                    }

                    let (oty, not_never) =
                        self.pick_types_from_sense(lhs_type, false, self.never_type); // Filter out falsy types

                    if not_never {
                        let oty = oty.unwrap();

                        // Perform a limited form of type reduction for booleans
                        if is_prim(oty, PrimitiveType::Boolean)
                            && self.is_boolean_singleton(rhs_type)
                        {
                            return self.boolean_type;
                        }
                        if is_prim(rhs_type, PrimitiveType::Boolean)
                            && self.is_boolean_singleton(oty)
                        {
                            return self.boolean_type;
                        }

                        self.union_of_types(oty, rhs_type, scope, &expr.base.base.location, false)
                    } else {
                        rhs_type
                    }
                }
            }

            AstExprBinary_Op::Or => {
                if lhs_is_any {
                    lhs_type
                } else {
                    let (oty, not_never) =
                        self.pick_types_from_sense(lhs_type, true, self.never_type); // Filter out truthy types

                    if not_never {
                        let oty = oty.unwrap();

                        // Perform a limited form of type reduction for booleans
                        if is_prim(oty, PrimitiveType::Boolean)
                            && self.is_boolean_singleton(rhs_type)
                        {
                            return self.boolean_type;
                        }
                        if is_prim(rhs_type, PrimitiveType::Boolean)
                            && self.is_boolean_singleton(oty)
                        {
                            return self.boolean_type;
                        }

                        self.union_of_types(oty, rhs_type, scope, &expr.base.base.location, true)
                    } else {
                        rhs_type
                    }
                }
            }

            _ => {
                self.ice_string_location(
                    &format!(
                        "checkRelationalOperation called with incorrect binary expression '{}'",
                        luaur_ast::functions::to_string_ast_alt_b::to_string(expr.op)
                    ),
                    &expr.base.base.location,
                );
                unreachable!()
            }
        }
    }

    /// C++ `stripNil` lambda inside `checkRelationalOperation` (L2721-2739).
    fn relational_strip_nil(&mut self, ty: TypeId, is_or_op: bool) -> TypeId {
        let ty = unsafe { follow_type_id(ty) };
        if !self.is_nonstrict_mode() && !is_or_op {
            return ty;
        }

        if unsafe { !get_type_id::<UnionType>(ty).is_null() } {
            let cleaned = self.try_strip_union_from_nil(ty);

            // If there is no union option without 'nil'
            match cleaned {
                None => return self.nil_type,
                Some(c) => return unsafe { follow_type_id(c) },
            }
        }

        unsafe { follow_type_id(ty) }
    }

    /// C++ `get<BooleanSingleton>(get<SingletonType>(follow(ty)))` truthiness check.
    fn is_boolean_singleton(&self, ty: TypeId) -> bool {
        unsafe {
            let stv = get_type_id::<SingletonType>(follow_type_id(ty));
            !stv.is_null() && BooleanSingleton::get_if(&(*stv).variant).is_some()
        }
    }
}
