use crate::enums::control_flow::ControlFlow;
use crate::records::module::Module;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use luaur_ast::records::ast_stat_error::AstStatError;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_error(
        &mut self,
        scope: &ScopePtr,
        error_statement: &AstStatError,
    ) -> ControlFlow {
        let module_ptr =
            alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
        let old_size = unsafe { (*module_ptr).errors.len() };

        for statement in error_statement.statements.iter() {
            self.check_scope_ptr_ast_stat(scope, unsafe { &**statement });
        }

        for expr in error_statement.expressions.iter() {
            self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &**expr },
                None,
                false,
            );
        }

        unsafe {
            (*module_ptr).errors.truncate(old_size);
        }

        ControlFlow::None
    }
}
