use crate::enums::control_flow::ControlFlow;
use crate::records::type_checker::TypeChecker;
use luaur_ast::records::ast_stat_while::AstStatWhile;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_while(
        &mut self,
        scope: &crate::type_aliases::scope_ptr_type_infer::ScopePtr,
        statement: &AstStatWhile,
    ) -> ControlFlow {
        let result = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
            scope,
            unsafe { &*statement.condition },
            None,
            false,
        );
        let while_scope = self.child_scope(scope, &unsafe { (*statement.body).base.base.location });
        self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &while_scope, true);
        self.check_scope_ptr_ast_stat_block(&while_scope, unsafe { &*statement.body });
        ControlFlow::None
    }
}
