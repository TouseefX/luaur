use crate::enums::control_flow::ControlFlow;
use crate::functions::add_all_as_dependencies::add_all_as_dependencies;
use crate::functions::checkpoint::checkpoint;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::for_each_constraint::for_each_constraint;
use crate::functions::get_mutable_type::getMutable;
use crate::functions::match_require::match_require;
use crate::functions::match_set_metatable::match_set_metatable;
use crate::records::binding::Binding;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::name_constraint::NameConstraint;
use crate::records::pack_subtype_constraint::PackSubtypeConstraint;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use crate::records::type_fun::TypeFun;
use crate::records::type_ids::TypeIds;
use crate::records::unpack_constraint::UnpackConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::CStr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::rtti::ast_node_as;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl ConstraintGenerator {
    pub fn visit_scope_ptr_ast_stat_local(
        &mut self,
        scope: &ScopePtr,
        stat_local: *mut AstStatLocal,
    ) -> ControlFlow {
        let stat_local_ref = unsafe { &*stat_local };
        let scope_raw = scope.as_ref() as *const Scope as *mut Scope;

        let mut annotated_types = Vec::new();
        annotated_types.reserve(stat_local_ref.vars.size);
        let mut has_annotation = false;

        let mut expected_types = Vec::new();
        expected_types.reserve(stat_local_ref.vars.size);

        let mut assignees = Vec::new();
        assignees.reserve(stat_local_ref.vars.size);

        let mut first_value_type = None;

        for i in 0..stat_local_ref.vars.size {
            let local: *mut AstLocal = unsafe { *stat_local_ref.vars.data.add(i) };
            let location = unsafe { (*local).location };

            let assignee = unsafe { (*self.arena).add_type(BlockedType::default()) };
            self.local_types.try_insert(assignee, TypeIds::type_ids());
            assignees.push(assignee);

            if first_value_type.is_none() {
                first_value_type = Some(assignee);
            }

            if !unsafe { (*local).annotation }.is_null() {
                has_annotation = true;
                let annotation_ty = self.resolve_type(
                    scope_raw,
                    unsafe { (*local).annotation },
                    false,
                    false,
                    crate::enums::polarity::Polarity::Positive,
                );
                annotated_types.push(annotation_ty);
                expected_types.push(Some(annotation_ty));
                unsafe {
                    (*scope_raw).bindings.insert(
                        Symbol::from_local(local),
                        Binding {
                            type_id: annotation_ty,
                            location,
                            deprecated: false,
                            deprecated_suggestion: String::new(),
                            documentation_symbol: None,
                        },
                    );
                }
            } else {
                annotated_types.push(unsafe { (*self.builtin_types).unknownType });
                expected_types.push(None);
                unsafe {
                    (*scope_raw).bindings.insert(
                        Symbol::from_local(local),
                        Binding {
                            type_id: (*self.builtin_types).unknownType,
                            location,
                            deprecated: false,
                            deprecated_suggestion: String::new(),
                            documentation_symbol: None,
                        },
                    );
                }

                let mut types = TypeIds::type_ids();
                types.insert_type_id(assignee);
                self.inferred_bindings.try_insert(
                    Symbol::from_local(local),
                    crate::records::constraint_generator::InferredBinding {
                        scope: scope_raw,
                        location,
                        types,
                    },
                );
            }

            let def = unsafe { (*self.dfg).get_def_local(local) };
            unsafe {
                *(*scope_raw).lvalue_types.get_or_insert(def) = assignee;
            }
        }

        let start = checkpoint(self as *const ConstraintGenerator);
        let rvalue_pack = self
            .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
                scope,
                stat_local_ref.values,
                &expected_types,
            )
            .tp;
        let end = checkpoint(self as *const ConstraintGenerator);

        let mut deferred_types = Vec::new();
        let (head, tail) = flatten_type_pack_id(rvalue_pack);
        let mut fresh_blocked_types: Vec<*mut BlockedType> = Vec::new();

        for i in 0..stat_local_ref.vars.size {
            LUAU_ASSERT!(unsafe { !getMutable::<BlockedType>(assignees[i]).is_null() });
            let local_domain = self
                .local_types
                .find_mut(&assignees[i])
                .expect("local assignee domain should exist");

            let local = unsafe { *stat_local_ref.vars.data.add(i) };
            if !unsafe { (*local).annotation }.is_null() {
                local_domain.insert_type_id(annotated_types[i]);
                if i >= head.len() && tail.is_some() {
                    deferred_types.push(annotated_types[i]);
                }
            } else if i < head.len() {
                local_domain.insert_type_id(head[i]);
            } else if tail.is_some() {
                let deferred = unsafe { (*self.arena).add_type(BlockedType::default()) };
                deferred_types.push(deferred);
                local_domain.insert_type_id(deferred);
                fresh_blocked_types.push(unsafe { getMutable::<BlockedType>(deferred) });
            } else {
                local_domain.insert_type_id(unsafe { (*self.builtin_types).nilType });
            }
        }

        if has_annotation {
            let annotated_pack = unsafe {
                (*self.arena)
                    .add_type_pack_vector_type_id_optional_type_pack_id(annotated_types, None)
            };
            self.add_constraint_scope_ptr_location_constraint_v(
                scope,
                stat_local_ref.base.base.location,
                ConstraintV::PackSubtype(PackSubtypeConstraint {
                    sub_pack: rvalue_pack,
                    super_pack: annotated_pack,
                    returns: false,
                }),
            );
        }

        if !deferred_types.is_empty() {
            LUAU_ASSERT!(tail.is_some());
            let uc = self.add_constraint_scope_ptr_location_constraint_v(
                scope,
                stat_local_ref.base.base.location,
                ConstraintV::Unpack(UnpackConstraint {
                    result_pack: deferred_types,
                    source_pack: tail.unwrap(),
                }),
            );

            if FFlag::LuauConstraintGraph.get() {
                add_all_as_dependencies(start, end, self, uc);
            } else {
                for_each_constraint(start, end, self, |run_before: *mut Constraint| unsafe {
                    (*uc).deprecated_dependencies.push(run_before);
                });
            }

            for bt in fresh_blocked_types {
                if !bt.is_null() {
                    unsafe { (*bt).setOwner(uc) };
                }
            }
        }

        if stat_local_ref.vars.size == 1
            && stat_local_ref.values.size == 1
            && first_value_type.is_some()
            && core::ptr::eq(scope_raw, self.root_scope)
            && !has_annotation
        {
            let var = unsafe { *stat_local_ref.vars.data };
            let value = unsafe { *stat_local_ref.values.data };
            let should_name = unsafe {
                let node = value as *mut AstNode;
                (*node).is::<AstExprTable>()
                    || ast_node_as::<AstExprCall>(node)
                        .as_ref()
                        .is_some_and(|call| match_set_metatable(call))
            };

            if should_name {
                let name = unsafe {
                    CStr::from_ptr((*var).name.value)
                        .to_string_lossy()
                        .into_owned()
                };
                self.add_constraint_scope_ptr_location_constraint_v(
                    scope,
                    unsafe { (*value).base.location },
                    ConstraintV::Name(NameConstraint {
                        named_type: first_value_type.unwrap(),
                        name,
                        synthetic: true,
                        type_parameters: Vec::new(),
                        type_pack_parameters: Vec::new(),
                    }),
                );
            }
        }

        if stat_local_ref.values.size > 0 {
            for i in 0..core::cmp::min(stat_local_ref.values.size, stat_local_ref.vars.size) {
                let value = unsafe { *stat_local_ref.values.data.add(i) };
                let call = unsafe { ast_node_as::<AstExprCall>(value as *mut AstNode).as_ref() };
                let Some(call) = call else {
                    continue;
                };
                let Some(require) = match_require(call) else {
                    continue;
                };

                let module_name = self.module.as_ref().unwrap().name.clone();
                let module_info = unsafe {
                    ((*self.module_resolver).vtable.resolve_module_info)(
                        self.module_resolver,
                        &module_name,
                        require as *const _,
                    )
                };
                let Some(module_info) = module_info else {
                    continue;
                };

                let required_module = unsafe {
                    ((*self.module_resolver).vtable.get_module)(
                        self.module_resolver,
                        &module_info.name,
                    )
                };
                let Some(required_module) = required_module else {
                    continue;
                };

                let local = unsafe { *stat_local_ref.vars.data.add(i) };
                let name = unsafe {
                    CStr::from_ptr((*local).name.value)
                        .to_string_lossy()
                        .into_owned()
                };
                unsafe {
                    (*scope_raw)
                        .imported_type_bindings
                        .insert(name.clone(), required_module.exported_type_bindings.clone());
                    (*scope_raw)
                        .imported_modules
                        .insert(name.clone(), module_info.name.clone());
                }

                for cycle in &self.require_cycles {
                    if cycle.path.is_empty() || cycle.path[0] != module_info.name {
                        continue;
                    }

                    unsafe {
                        if let Some(bindings) = (*scope_raw).imported_type_bindings.get_mut(&name) {
                            for (_, tf) in bindings.iter_mut() {
                                *tf = TypeFun {
                                    type_params: Vec::new(),
                                    type_pack_params: Vec::new(),
                                    r#type: (*self.builtin_types).anyType,
                                    definition_location: None,
                                };
                            }
                        }
                    }
                }
            }
        }

        ControlFlow::None
    }
}
