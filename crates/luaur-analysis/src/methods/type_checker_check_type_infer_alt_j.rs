use crate::enums::control_flow::ControlFlow;
use crate::functions::first::first;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::match_require::match_require;
use crate::functions::match_set_metatable::match_set_metatable;
use crate::functions::unwrap_group::unwrap_group;
use crate::records::binding::Binding;
use crate::records::count_mismatch::CountMismatchContext;
use crate::records::metatable_type::MetatableType;
use crate::records::symbol::Symbol;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_fun::TypeFun;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use core::ffi::CStr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::rtti::ast_node_as;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_local(
        &mut self,
        scope: &ScopePtr,
        local: &AstStatLocal,
    ) -> ControlFlow {
        let mut variable_types: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();
        let mut expected_types: alloc::vec::Vec<Option<TypeId>> = alloc::vec::Vec::new();
        let mut bindings: alloc::vec::Vec<(*mut luaur_ast::records::ast_local::AstLocal, Binding)> =
            alloc::vec::Vec::new();

        variable_types.reserve(local.vars.size);
        expected_types.reserve(local.vars.size);
        bindings.reserve(local.vars.size);

        for i in 0..local.vars.size {
            let var = unsafe { *local.vars.data.add(i) };
            let annotation = unsafe { (*var).annotation };
            let rhs_is_table = i < local.values.size
                && unsafe {
                    !ast_node_as::<AstExprTable>(*local.values.data.add(i) as *mut AstNode)
                        .is_null()
                };

            let mut ty: TypeId = core::ptr::null_mut();
            if !annotation.is_null() {
                ty = self.resolve_type(scope.clone(), unsafe { &*annotation });
                if unsafe { !get_type_id::<ErrorType>(follow_type_id(ty)).is_null() } {
                    ty = core::ptr::null_mut();
                }
            }

            if ty.is_null() {
                ty = if rhs_is_table || !self.is_nonstrict_mode() {
                    self.fresh_type_scope_ptr(scope.clone())
                } else {
                    self.any_type
                };
            }

            variable_types.push(ty);
            expected_types.push(Some(ty));
            bindings.push((
                var,
                Binding {
                    type_id: ty,
                    location: unsafe { (*var).location },
                    deprecated: false,
                    deprecated_suggestion: alloc::string::String::new(),
                    documentation_symbol: None,
                },
            ));
        }

        if local.values.size > 0 {
            let variable_tail = self.fresh_type_pack_scope_ptr(scope.clone());
            let variable_pack = self.add_type_pack_vector_type_id_optional_type_pack_id(
                &variable_types,
                Some(variable_tail),
            );
            let value_pack = self
                .check_expr_list(
                    scope,
                    &local.base.base.location,
                    &local.values,
                    true,
                    &alloc::vec::Vec::new(),
                    &expected_types,
                )
                .r#type;

            let mut ctx = CountMismatchContext::ExprListResult;
            if local.values.size == 1 {
                let expr = unsafe { unwrap_group(*local.values.data.add(0)) };
                if unsafe { !ast_node_as::<AstExprCall>(expr as *mut AstNode).is_null() } {
                    ctx = CountMismatchContext::FunctionResult;
                }
            }

            self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
                value_pack,
                variable_pack,
                scope,
                &local.base.base.location,
                ctx,
            );

            if local.vars.size == 1 && local.values.size == 1 {
                unsafe {
                    let rhs = *local.values.data.add(0);
                    if let Some(ty) = first(value_pack, true) {
                        if !ast_node_as::<AstExprTable>(rhs as *mut AstNode).is_null() {
                            let ttv = get_mutable_type_id::<TableType>(follow_type_id(ty));
                            if !ttv.is_null() && (*ttv).name.is_none() {
                                if let Some(current_module) = self.current_module.as_ref() {
                                    let module_scope = current_module.get_module_scope();
                                    if alloc::sync::Arc::ptr_eq(scope, &module_scope) {
                                        let var = *local.vars.data.add(0);
                                        if !(*var).name.value.is_null() {
                                            (*ttv).synthetic_name = Some(
                                                CStr::from_ptr((*var).name.value)
                                                    .to_string_lossy()
                                                    .into_owned(),
                                            );
                                        }
                                    }
                                }
                            }
                        } else {
                            let call = ast_node_as::<AstExprCall>(rhs as *mut AstNode);
                            if !call.is_null() && match_set_metatable(&*call) {
                                let mtv = get_mutable_type_id::<MetatableType>(follow_type_id(ty));
                                if !mtv.is_null() {
                                    let var = *local.vars.data.add(0);
                                    if !(*var).name.value.is_null() {
                                        (*mtv).syntheticName = Some(
                                            CStr::from_ptr((*var).name.value)
                                                .to_string_lossy()
                                                .into_owned(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        unsafe {
            let scope_mut = alloc::sync::Arc::as_ptr(scope) as *mut crate::records::scope::Scope;
            for i in 0..core::cmp::min(local.values.size, local.vars.size) {
                let value = *local.values.data.add(i);
                let call = ast_node_as::<AstExprCall>(value as *mut AstNode);
                if call.is_null() {
                    continue;
                }

                let Some(require) = match_require(&*call) else {
                    continue;
                };

                let Some(current_module) = self.current_module.as_ref() else {
                    continue;
                };

                let module_info = ((*self.resolver).vtable.resolve_module_info)(
                    self.resolver,
                    &current_module.name,
                    require,
                );

                let Some(module_info) = module_info else {
                    continue;
                };

                let var = *local.vars.data.add(i);
                if (*var).name.value.is_null() {
                    continue;
                }

                let name: Name = CStr::from_ptr((*var).name.value)
                    .to_string_lossy()
                    .into_owned();

                if let Some(module) =
                    ((*self.resolver).vtable.get_module)(self.resolver, &module_info.name)
                {
                    (*scope_mut)
                        .imported_type_bindings
                        .insert(name.clone(), module.exported_type_bindings.clone());
                    (*scope_mut)
                        .imported_modules
                        .insert(name.clone(), module_info.name.clone());

                    for require_cycle in &self.require_cycles {
                        if !require_cycle.path.is_empty()
                            && require_cycle.path[0] == module_info.name
                        {
                            if let Some(imported_bindings) =
                                (*scope_mut).imported_type_bindings.get_mut(&name)
                            {
                                for type_fun in imported_bindings.values_mut() {
                                    *type_fun = TypeFun::type_fun_type_id(self.any_type);
                                }
                            }
                        }
                    }
                }
            }

            for (local, binding) in bindings {
                (*scope_mut)
                    .bindings
                    .insert(Symbol::from_local(local), binding);
            }
        }

        ControlFlow::None
    }
}
