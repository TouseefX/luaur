use crate::enums::control_flow::ControlFlow;
use crate::enums::table_state::TableState;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_table_type::get_mutable_table_type;
use crate::functions::is_table_intersection::is_table_intersection;
use crate::records::binding::Binding;
use crate::records::cannot_extend_table::CannotExtendTable;
use crate::records::only_tables_can_have_methods::OnlyTablesCanHaveMethods;
use crate::records::property_type::Property;
use crate::records::symbol::Symbol;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::rtti::ast_node_as;

impl TypeChecker {
    pub fn check_scope_ptr_type_id_scope_ptr_ast_stat_function(
        &mut self,
        scope: &ScopePtr,
        mut ty: TypeId,
        fun_scope: &ScopePtr,
        function: &AstStatFunction,
    ) -> ControlFlow {
        let name_node = function.name as *mut AstNode;

        let expr_name = unsafe { ast_node_as::<AstExprGlobal>(name_node) };
        if !expr_name.is_null() {
            let expr_name = unsafe { &*expr_name };
            let module_scope = self.current_module.as_ref().unwrap().get_module_scope();
            let name = Symbol::from_global(expr_name.name);
            let previously_defined =
                self.is_nonstrict_mode() && module_scope.bindings.contains_key(&name);
            let old_binding = if previously_defined {
                module_scope.bindings.get(&name).cloned()
            } else {
                None
            };

            unsafe {
                let module_scope_mut =
                    alloc::sync::Arc::as_ptr(&module_scope) as *mut crate::records::scope::Scope;
                (*module_scope_mut).bindings.insert(
                    name.clone(),
                    Binding {
                        type_id: ty,
                        location: expr_name.base.base.location,
                        deprecated: false,
                        deprecated_suggestion: String::new(),
                        documentation_symbol: None,
                    },
                );
            }

            self.check_function_body(fun_scope, ty, unsafe { &*function.func });

            let final_binding = if let Some(old) = old_binding {
                old
            } else {
                Binding {
                    type_id: self.quantify(fun_scope, ty, expr_name.base.base.location),
                    location: expr_name.base.base.location,
                    deprecated: false,
                    deprecated_suggestion: String::new(),
                    documentation_symbol: None,
                }
            };

            unsafe {
                let module_scope_mut =
                    alloc::sync::Arc::as_ptr(&module_scope) as *mut crate::records::scope::Scope;
                (*module_scope_mut).bindings.insert(name, final_binding);
            }

            return ControlFlow::None;
        }

        let local_name = unsafe { ast_node_as::<AstExprLocal>(name_node) };
        if !local_name.is_null() {
            let local_name = unsafe { &*local_name };
            let symbol = Symbol::from_local(local_name.local);
            let name_location = unsafe { (*local_name.local).location };

            unsafe {
                let scope_mut =
                    alloc::sync::Arc::as_ptr(scope) as *mut crate::records::scope::Scope;
                (*scope_mut).bindings.insert(
                    symbol.clone(),
                    Binding {
                        type_id: ty,
                        location: name_location,
                        deprecated: false,
                        deprecated_suggestion: String::new(),
                        documentation_symbol: None,
                    },
                );
            }

            self.check_function_body(fun_scope, ty, unsafe { &*function.func });

            let quantified_ty = self.quantify(fun_scope, ty, name_location);
            let quantified = self.any_if_nonstrict(quantified_ty);
            unsafe {
                let scope_mut =
                    alloc::sync::Arc::as_ptr(scope) as *mut crate::records::scope::Scope;
                (*scope_mut).bindings.insert(
                    symbol,
                    Binding {
                        type_id: quantified,
                        location: name_location,
                        deprecated: false,
                        deprecated_suggestion: String::new(),
                        documentation_symbol: None,
                    },
                );
            }

            return ControlFlow::None;
        }

        let index_name = unsafe { ast_node_as::<AstExprIndexName>(name_node) };
        if !index_name.is_null() {
            let index_name = unsafe { &*index_name };
            let expr_ty = self
                .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                    scope,
                    unsafe { &*index_name.expr },
                    None,
                    false,
                )
                .r#type;
            let ttv = get_mutable_table_type(expr_ty);
            let prop_name = unsafe {
                core::ffi::CStr::from_ptr(index_name.index.value)
                    .to_string_lossy()
                    .into_owned()
            };

            if self
                .get_index_type_from_type(
                    scope.clone(),
                    expr_ty,
                    &prop_name,
                    &index_name.index_location,
                    false,
                )
                .is_none()
            {
                let data = if !ttv.is_null() || is_table_intersection(expr_ty) {
                    TypeErrorData::CannotExtendTable(CannotExtendTable {
                        table_type: expr_ty,
                        context: crate::records::cannot_extend_table::Context::Property,
                        prop: prop_name.clone(),
                    })
                } else {
                    TypeErrorData::OnlyTablesCanHaveMethods(OnlyTablesCanHaveMethods {
                        table_type: expr_ty,
                    })
                };

                self.report_error_location_type_error_data(&function.base.base.location, data);
            }

            ty = unsafe { follow_type_id(ty) };
            if !ttv.is_null() && unsafe { (*ttv).state != TableState::Sealed } {
                unsafe {
                    let property = (*ttv)
                        .props
                        .entry(prop_name.clone())
                        .or_insert_with(Property::default);
                    property.set_type(ty);
                    property.location = Some(index_name.index_location);
                }
            }

            self.check_function_body(fun_scope, ty, unsafe { &*function.func });

            if !ttv.is_null() && unsafe { (*ttv).state != TableState::Sealed } {
                let quantified = unsafe {
                    follow_type_id(self.quantify(fun_scope, ty, index_name.index_location))
                };
                unsafe {
                    let property = (*ttv)
                        .props
                        .entry(prop_name)
                        .or_insert_with(Property::default);
                    property.set_type(quantified);
                    property.location = Some(index_name.index_location);
                }
            }

            return ControlFlow::None;
        }

        self.check_function_body(fun_scope, ty, unsafe { &*function.func });
        ControlFlow::None
    }
}
