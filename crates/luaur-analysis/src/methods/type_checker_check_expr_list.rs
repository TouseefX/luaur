use crate::records::type_checker::TypeChecker;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::location::Location;

use crate::functions::contains_never::contains_never;
use crate::functions::first::first;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::txn_log::TxnLog;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::predicate_vec::PredicateVec;

impl TypeChecker {
    pub fn check_expr_list(
        &mut self,
        scope: &ScopePtr,
        location: &Location,
        exprs: &AstArray<*mut AstExpr>,
        substitute_free_for_nil: bool,
        instantiate_generics: &alloc::vec::Vec<bool>,
        expected_types: &alloc::vec::Vec<Option<TypeId>>,
    ) -> WithPredicate<TypePackId> {
        let mut uninhabitable = false;
        let pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
            head: alloc::vec::Vec::new(),
            tail: None,
        }));
        let mut predicates: PredicateVec = alloc::vec::Vec::new(); // At the moment we will be pushing all predicate sets into this. Do we need some way to split them up?

        if exprs.size == 0 {
            return WithPredicate::with_predicate_t(pack);
        }

        let tp = unsafe { get_mutable_type_pack_id::<TypePack>(pack) };

        let last_index = exprs.size - 1;
        unsafe { (*tp).head.reserve(last_index) };

        let mut state = self.mk_unifier(scope, location);

        let mut inverse_logs: alloc::vec::Vec<TxnLog> = alloc::vec::Vec::new();

        for i in 0..exprs.size {
            let expr = unsafe { *exprs.data.add(i) };
            let expected_type: Option<TypeId> = if i < expected_types.len() {
                expected_types[i]
            } else {
                None
            };

            let is_call_or_varargs = unsafe {
                (*expr).base.class_index == crate::rtti::ast_rtti_index("AstExprCall")
                    || (*expr).base.class_index == crate::rtti::ast_rtti_index("AstExprVarargs")
            };

            if i == last_index && is_call_or_varargs {
                let result = self.check_expr_pack(scope, unsafe { &*expr });
                let type_pack = result.r#type;
                for c in result.predicates {
                    predicates.push(c);
                }

                if contains_never(type_pack) {
                    // f(), g() where f() returns (never, string) or (string, never) means this whole TypePackId is uninhabitable, so return (never,
                    // ...never)
                    uninhabitable = true;
                    continue;
                } else if let Some(first_ty) = first(type_pack, true) {
                    let module_ptr = alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                        as *mut crate::records::module::Module;
                    let key = expr as *const AstExpr;
                    if unsafe { (*module_ptr).ast_types.find(&key).is_none() } {
                        unsafe {
                            *(*module_ptr).ast_types.get_or_insert(key) = follow_type_id(first_ty);
                        }
                    }
                }

                if let Some(expected_type) = expected_type {
                    let module_ptr = alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                        as *mut crate::records::module::Module;
                    let key = expr as *const AstExpr;
                    unsafe {
                        *(*module_ptr).ast_expected_types.get_or_insert(key) = expected_type;
                    }
                }

                unsafe { (*tp).tail = Some(type_pack) };
            } else {
                let result = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                    scope,
                    unsafe { &*expr },
                    expected_type,
                    false,
                );
                let r#type = result.r#type;
                for c in result.predicates {
                    predicates.push(c);
                }

                if unsafe { !get_type_id::<NeverType>(r#type).is_null() } {
                    // f(), g() where f() returns (never, string) or (string, never) means this whole TypePackId is uninhabitable, so return (never,
                    // ...never)
                    uninhabitable = true;
                    continue;
                }

                let is_constant_nil = unsafe {
                    (*expr).base.class_index == crate::rtti::ast_rtti_index("AstExprConstantNil")
                };
                let mut actual_type = if substitute_free_for_nil && is_constant_nil {
                    self.fresh_type_scope_ptr(scope.clone())
                } else {
                    r#type
                };

                if !luaur_common::FFlag::LuauInstantiateInSubtyping.get() {
                    if instantiate_generics.len() > i && instantiate_generics[i] {
                        let loc = unsafe { (*expr).base.location };
                        actual_type = self.instantiate(scope, actual_type, loc, TxnLog::empty());
                    }
                }

                if let Some(expected_type) = expected_type {
                    state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
                        actual_type,
                        expected_type,
                        false,
                        false,
                        None,
                    );

                    // Ugly: In future iterations of the loop, we might need the state of the unification we
                    // just performed. There's not a great way to pass that into checkExpr. Instead, we store
                    // the inverse of the current log, and commit it. When we're done, we'll commit all the
                    // inverses. This isn't optimal, and a better solution is welcome here.
                    inverse_logs.push(state.log.inverse());
                    state.log.commit();
                }

                unsafe { (*tp).head.push(actual_type) };
            }
        }

        for log in inverse_logs.iter_mut() {
            log.commit();
        }

        if uninhabitable {
            return WithPredicate::with_predicate_t(self.uninhabitable_type_pack);
        }
        WithPredicate::with_predicate_t_predicate_vec(pack, predicates)
    }
}
