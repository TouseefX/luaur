use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::binding::Binding;
use crate::records::never_type::NeverType;
use crate::records::symbol::Symbol;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use crate::records::unknown_symbol::Context;
use crate::records::unknown_symbol::UnknownSymbol;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl TypeChecker {
    pub fn check_l_value_binding_scope_ptr_ast_expr_global(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprGlobal,
    ) -> TypeId {
        let name: Name = unsafe {
            core::ffi::CStr::from_ptr(expr.name.value)
                .to_string_lossy()
                .into_owned()
        };
        let module_scope = self.current_module.as_ref().unwrap().get_module_scope();

        let sym = Symbol::from_global(expr.name);

        if let Some(binding) = module_scope.bindings.get(&sym) {
            let ty = unsafe { follow_type_id(binding.type_id) };
            if unsafe { !get_type_id::<NeverType>(ty).is_null() } {
                return self.unknown_type;
            }
            return ty;
        }

        let result = self.fresh_type_scope_ptr(scope.clone());

        {
            let module_scope_ptr =
                alloc::sync::Arc::as_ptr(&module_scope) as *mut crate::records::scope::Scope;
            let binding = Binding {
                type_id: result,
                location: expr.base.base.location,
                deprecated: false,
                deprecated_suggestion: alloc::string::String::new(),
                documentation_symbol: None,
            };
            unsafe {
                (*module_scope_ptr).bindings.insert(sym, binding);
            }
        }

        // If we're in strict mode, we want to report defining a global as an error,
        // but still add it to the bindings, so that autocomplete includes it in completions.
        if !self.is_nonstrict_mode() {
            let error_data =
                TypeErrorData::UnknownSymbol(UnknownSymbol::new(name, Context::Binding));
            let error =
                TypeError::type_error_location_type_error_data(expr.base.base.location, error_data);
            self.report_error_type_error(&error);
        }

        result
    }
}
