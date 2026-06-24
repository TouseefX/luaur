// ConstraintGenerator::visit(const ScopePtr&, AstStatForIn*) (ConstraintGenerator.cpp:1587-1691).
use crate::enums::control_flow::ControlFlow;
use crate::functions::add_all_as_reverse_dependencies::add_all_as_reverse_dependencies;
use crate::functions::checkpoint::checkpoint;
use crate::functions::for_each_constraint::for_each_constraint;
use crate::functions::get_mutable_type::getMutable;
use crate::records::binding::Binding;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::iterable_constraint::IterableConstraint;
use crate::records::module::Module;
use crate::records::reduce_constraint::ReduceConstraint;
use crate::records::scope::Scope;
use crate::records::subtype_constraint::SubtypeConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::rc::Rc;
use alloc::vec::Vec;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl ConstraintGenerator {
    pub fn visit_scope_ptr_ast_stat_for_in(
        &mut self,
        scope: &ScopePtr,
        for_in: *mut AstStatForIn,
    ) -> ControlFlow {
        let for_in_ref = unsafe { &*for_in };

        let loop_scope: ScopePtr = self.child_scope(
            &for_in_ref.base.base as *const AstNode as *mut AstNode,
            scope,
        );
        let loop_scope_raw = loop_scope.as_ref() as *const Scope as *mut Scope;

        let values: AstArray<*mut AstExpr> = for_in_ref.values;
        let iterator: TypePackId = self
            .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
                scope,
                values,
                &Vec::new(),
            )
            .tp;

        let mut variable_types: Vec<TypeId> = Vec::new();
        variable_types.reserve(for_in_ref.vars.size);

        for i in 0..for_in_ref.vars.size {
            let var = unsafe { *for_in_ref.vars.data.add(i) };
            let loop_var: TypeId = unsafe { (*self.arena).add_type(BlockedType::default()) };
            variable_types.push(loop_var);

            if FFlag::LuauPropagateTypeAnnotationsInForInLoops.get() {
                let def = unsafe { (*self.dfg).get_def_local(var) };

                if !unsafe { (*var).annotation }.is_null() {
                    let annotation_ty = self.resolve_type(
                        loop_scope.as_ref() as *const Scope as *mut Scope,
                        unsafe { (*var).annotation },
                        /* in_type_arguments */ false,
                        /* replace_error_with_fresh */ false,
                        crate::enums::polarity::Polarity::Positive,
                    );
                    unsafe {
                        (*loop_scope_raw).bindings.insert(
                            crate::records::symbol::Symbol::from_local(var),
                            Binding {
                                type_id: annotation_ty,
                                location: (*var).location,
                                deprecated: false,
                                deprecated_suggestion: alloc::string::String::new(),
                                documentation_symbol: None,
                            },
                        );
                    }
                    self.add_constraint_scope_ptr_location_constraint_v(
                        scope,
                        unsafe { (*var).location },
                        ConstraintV::Subtype(SubtypeConstraint {
                            sub_type: loop_var,
                            super_type: annotation_ty,
                        }),
                    );
                    unsafe {
                        *(*loop_scope_raw).lvalue_types.get_or_insert(def) = annotation_ty;
                    }
                } else {
                    unsafe {
                        (*loop_scope_raw).bindings.insert(
                            crate::records::symbol::Symbol::from_local(var),
                            Binding {
                                type_id: loop_var,
                                location: (*var).location,
                                deprecated: false,
                                deprecated_suggestion: alloc::string::String::new(),
                                documentation_symbol: None,
                            },
                        );
                        *(*loop_scope_raw).lvalue_types.get_or_insert(def) = loop_var;
                    }
                }
            } else {
                if !unsafe { (*var).annotation }.is_null() {
                    let annotation_ty = self.resolve_type(
                        loop_scope.as_ref() as *const Scope as *mut Scope,
                        unsafe { (*var).annotation },
                        /* in_type_arguments */ false,
                        /* replace_error_with_fresh */ false,
                        crate::enums::polarity::Polarity::Positive,
                    );
                    unsafe {
                        (*loop_scope_raw).bindings.insert(
                            crate::records::symbol::Symbol::from_local(var),
                            Binding {
                                type_id: annotation_ty,
                                location: (*var).location,
                                deprecated: false,
                                deprecated_suggestion: alloc::string::String::new(),
                                documentation_symbol: None,
                            },
                        );
                    }
                    self.add_constraint_scope_ptr_location_constraint_v(
                        scope,
                        unsafe { (*var).location },
                        ConstraintV::Subtype(SubtypeConstraint {
                            sub_type: loop_var,
                            super_type: annotation_ty,
                        }),
                    );
                } else {
                    unsafe {
                        (*loop_scope_raw).bindings.insert(
                            crate::records::symbol::Symbol::from_local(var),
                            Binding {
                                type_id: loop_var,
                                location: (*var).location,
                                deprecated: false,
                                deprecated_suggestion: alloc::string::String::new(),
                                documentation_symbol: None,
                            },
                        );
                    }
                }

                let def = unsafe { (*self.dfg).get_def_local(var) };
                unsafe {
                    *(*loop_scope_raw).lvalue_types.get_or_insert(def) = loop_var;
                }
            }
        }

        let next_ast_fragment = unsafe { *values.data.add(0) } as *const AstNode;
        let ast_for_in_next_types: *mut luaur_common::records::dense_hash_map::DenseHashMap<
            *const AstNode,
            TypeId,
        > = {
            let module = self.module.as_ref().unwrap();
            let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
            unsafe { &mut (*module_ptr).ast_for_in_next_types as *mut _ }
        };

        // C++ getLocation(forIn->values): span from first expr begin to last expr end.
        let values_location = {
            let first = unsafe { *values.data.add(0) };
            let last = unsafe { *values.data.add(values.size - 1) };
            Location {
                begin: unsafe { (*first).base.location.begin },
                end: unsafe { (*last).base.location.end },
            }
        };

        let iterable: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
            &loop_scope,
            values_location,
            ConstraintV::Iterable(IterableConstraint {
                iterator,
                variables: variable_types.clone(),
                next_ast_fragment,
                ast_for_in_next_types,
            }),
        );

        // Add an intersection ReduceConstraint for the key variable to denote that it can't be nil
        let key_var = unsafe { *for_in_ref.vars.data.add(0) };
        let key_def = unsafe { (*self.dfg).get_def_local(key_var) };
        let loop_var: TypeId = unsafe { *(*loop_scope_raw).lvalue_types.get_or_insert(key_def) };

        let intersection_ty: TypeId = {
            let intersect_func: &crate::records::type_function::TypeFunction =
                unsafe { &(*self.builtin_types).typeFunctions.intersect_func };
            let not_nil_ty = unsafe { (*self.builtin_types).notNilType };
            self.create_type_function_instance(
                intersect_func,
                alloc::vec![loop_var, not_nil_ty],
                Vec::new(),
                &loop_scope,
                unsafe { (*key_var).location },
            )
        };

        unsafe {
            (*loop_scope_raw).bindings.insert(
                crate::records::symbol::Symbol::from_local(key_var),
                Binding {
                    type_id: intersection_ty,
                    location: (*key_var).location,
                    deprecated: false,
                    deprecated_suggestion: alloc::string::String::new(),
                    documentation_symbol: None,
                },
            );
            *(*loop_scope_raw).lvalue_types.get_or_insert(key_def) = intersection_ty;
        }

        let c: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
            &loop_scope,
            unsafe { (*key_var).location },
            ConstraintV::Reduce(ReduceConstraint {
                ty: intersection_ty,
            }),
        );
        if FFlag::LuauConstraintGraph.get() {
            unsafe {
                (*self.cgraph).add_dependency_of_constraint_constraint(&mut *iterable, &mut *c)
            };
        } else {
            unsafe { (*c).deprecated_dependencies.push(iterable) };
        }

        for var in &variable_types {
            let bt = unsafe { getMutable::<BlockedType>(*var) };
            LUAU_ASSERT!(!bt.is_null());
            unsafe { (*bt).set_owner(iterable as *const _) };
        }

        let start = checkpoint(self as *const _);
        self.visit_scope_ptr_ast_stat_block(&loop_scope, for_in_ref.body);
        let end = checkpoint(self as *const _);

        unsafe {
            (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_assignments(&loop_scope);
        }

        // This iter constraint must dispatch first.
        if FFlag::LuauConstraintGraph.get() {
            add_all_as_reverse_dependencies(start, end, self, iterable);
        } else {
            for_each_constraint(start, end, self, |run_later: *mut Constraint| {
                unsafe { (*run_later).deprecated_dependencies.push(iterable) };
            });
        }

        ControlFlow::None
    }
}
