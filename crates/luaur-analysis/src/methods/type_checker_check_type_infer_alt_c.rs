use crate::enums::control_flow::ControlFlow;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use luaur_ast::records::ast_stat_block::AstStatBlock;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_block(
        &mut self,
        scope: &ScopePtr,
        block: &AstStatBlock,
    ) -> ControlFlow {
        let child = self.child_scope(scope, &block.base.base.location);
        let flow = self.check_block(&child, block);

        unsafe {
            let scope_mut = alloc::sync::Arc::as_ptr(scope) as *mut crate::records::scope::Scope;
            (*scope_mut).inherit_refinements(&child);
        }

        flow
    }
}
