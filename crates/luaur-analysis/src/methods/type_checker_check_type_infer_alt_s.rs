use crate::enums::control_flow::ControlFlow;
use crate::records::binding::Binding;
use crate::records::module::Module;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use alloc::string::String;
use alloc::sync::Arc;
use core::ffi::CStr;
use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_declare_global(
        &mut self,
        scope: &ScopePtr,
        global: &AstStatDeclareGlobal,
    ) -> ControlFlow {
        let global_ty = if global.type_.is_null() {
            self.error_recovery_type_scope_ptr(scope)
        } else {
            self.resolve_type(scope.clone(), unsafe { &*global.type_ })
        };
        let global_name = unsafe { CStr::from_ptr(global.name.value) }
            .to_string_lossy()
            .into_owned();

        unsafe {
            let module =
                Arc::as_ptr(self.current_module.as_ref().expect("current_module")) as *mut Module;
            (*module).declared_globals.insert(global_name, global_ty);

            let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
            (*scope_raw).bindings.insert(
                Symbol::from_global(global.name),
                Binding {
                    type_id: global_ty,
                    location: global.base.base.location,
                    deprecated: false,
                    deprecated_suggestion: String::new(),
                    documentation_symbol: None,
                },
            );
        }

        ControlFlow::None
    }
}
