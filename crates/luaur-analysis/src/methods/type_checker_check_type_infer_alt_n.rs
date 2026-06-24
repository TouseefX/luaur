use crate::enums::control_flow::ControlFlow;
use crate::records::binding::Binding;
use crate::records::symbol::Symbol;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;

impl TypeChecker {
    pub fn check_scope_ptr_type_id_scope_ptr_ast_stat_local_function(
        &mut self,
        scope: &ScopePtr,
        ty: TypeId,
        fun_scope: &ScopePtr,
        function: &AstStatLocalFunction,
    ) -> ControlFlow {
        // Name name = function.name->name.value; (declared in C++ parity, unused here)
        let _name = unsafe { (*function.name).name.value };

        unsafe {
            let scope_mut = alloc::sync::Arc::as_ptr(scope) as *mut crate::records::scope::Scope;
            (*scope_mut).bindings.insert(
                Symbol::from_local(function.name),
                Binding {
                    type_id: ty,
                    location: function.base.base.location,
                    deprecated: false,
                    deprecated_suggestion: alloc::string::String::new(),
                    documentation_symbol: None,
                },
            );
        }

        self.check_function_body(fun_scope, ty, unsafe { &*function.func });

        let name_location = unsafe { (*function.name).location };
        let quantified = self.quantify(fun_scope, ty, name_location);
        unsafe {
            let scope_mut = alloc::sync::Arc::as_ptr(scope) as *mut crate::records::scope::Scope;
            (*scope_mut).bindings.insert(
                Symbol::from_local(function.name),
                Binding {
                    type_id: quantified,
                    location: name_location,
                    deprecated: false,
                    deprecated_suggestion: alloc::string::String::new(),
                    documentation_symbol: None,
                },
            );
        }

        ControlFlow::None
    }
}
