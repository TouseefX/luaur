use crate::enums::table_state::TableState;
use crate::functions::get_mutable_table_type::get_mutable_table_type;
use crate::records::binding::Binding;
use crate::records::property_type::Property;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use crate::records::type_checker::TypeChecker;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_error::AstExprError;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl TypeChecker {
    // Answers the question: "Can I define another function with this name?"
    // Primarily about detecting duplicates.
    pub fn check_function_name(
        &mut self,
        scope: &ScopePtr,
        fun_name: &AstExpr,
        level: TypeLevel,
    ) -> TypeId {
        // auto freshTy = [&]() { return freshType(level); };

        let node = fun_name as *const AstExpr as *mut AstExpr as *mut AstNode;

        let global_name = unsafe { ast_node_as::<AstExprGlobal>(node) };
        if !global_name.is_null() {
            let global_name = unsafe { &*global_name };
            let module_scope = self.current_module.as_ref().unwrap().get_module_scope();
            let name = Symbol::from_global(global_name.name);
            if module_scope.bindings.contains_key(&name) {
                if self.is_nonstrict_mode() {
                    return module_scope.bindings.get(&name).unwrap().type_id;
                }

                return self.error_recovery_type_scope_ptr(scope);
            } else {
                let ty = self.fresh_type_type_level(level);
                let module_scope_ptr = alloc::sync::Arc::as_ptr(&module_scope) as *mut Scope;
                let binding = Binding {
                    type_id: ty,
                    location: fun_name.base.location,
                    deprecated: false,
                    deprecated_suggestion: alloc::string::String::new(),
                    documentation_symbol: None,
                };
                unsafe {
                    (*module_scope_ptr).bindings.insert(name, binding);
                }
                return ty;
            }
        }

        let local_name = unsafe { ast_node_as::<AstExprLocal>(node) };
        if !local_name.is_null() {
            let local_name = unsafe { &*local_name };
            let name = Symbol::from_local(local_name.local);
            let scope_ptr = alloc::sync::Arc::as_ptr(scope) as *mut Scope;
            // Binding& binding = scope->bindings[name];  — default-constructs (typeId == nullptr) if absent.
            let binding = unsafe {
                (*scope_ptr)
                    .bindings
                    .entry(name)
                    .or_insert_with(|| Binding {
                        type_id: core::ptr::null(),
                        location: fun_name.base.location,
                        deprecated: false,
                        deprecated_suggestion: alloc::string::String::new(),
                        documentation_symbol: None,
                    })
            };
            if binding.type_id.is_null() {
                *binding = Binding {
                    type_id: self.fresh_type_type_level(level),
                    location: fun_name.base.location,
                    deprecated: false,
                    deprecated_suggestion: alloc::string::String::new(),
                    documentation_symbol: None,
                };
            }
            return binding.type_id;
        }

        let index_name = unsafe { ast_node_as::<AstExprIndexName>(node) };
        if !index_name.is_null() {
            let index_name = unsafe { &*index_name };
            let lhs_type = self
                .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                    scope,
                    unsafe { &*index_name.expr },
                    None,
                    false,
                )
                .r#type;
            let ttv = get_mutable_table_type(lhs_type);

            if ttv.is_null() || unsafe { (*ttv).state } == TableState::Sealed {
                let name: Name = unsafe {
                    core::ffi::CStr::from_ptr(index_name.index.value)
                        .to_string_lossy()
                        .into_owned()
                };
                if let Some(ty) = self.get_index_type_from_type(
                    scope.clone(),
                    lhs_type,
                    &name,
                    &index_name.index_location,
                    false,
                ) {
                    return ty;
                }

                return self.error_recovery_type_scope_ptr(scope);
            }

            let name: Name = unsafe {
                core::ffi::CStr::from_ptr(index_name.index.value)
                    .to_string_lossy()
                    .into_owned()
            };

            let ttv = unsafe { &mut *ttv };

            if ttv.props.contains_key(&name) {
                return ttv.props.get(&name).unwrap().type_deprecated();
            }

            let fresh = self.fresh_type_type_level(level);
            let property = ttv.props.entry(name).or_insert_with(Property::property);
            property.set_type(fresh);
            property.location = Some(index_name.index_location);
            return property.type_deprecated();
        }

        if !unsafe { ast_node_as::<AstExprError>(node) }.is_null() {
            return self.error_recovery_type_scope_ptr(scope);
        }

        self.ice_string_location("Unexpected AST node type", &fun_name.base.location);
        self.error_recovery_type_scope_ptr(scope)
    }
}
