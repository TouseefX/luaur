use crate::enums::control_flow::ControlFlow;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::scope::Scope;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_stat_block::AstStatBlock;

impl ConstraintGenerator {
    // ConstraintGenerator::visit(const ScopePtr&, AstStatBlock*) (ConstraintGenerator.cpp).
    pub fn visit_scope_ptr_ast_stat_block(
        &mut self,
        scope: &ScopePtr,
        block: *mut AstStatBlock,
    ) -> ControlFlow {
        let inner_scope =
            self.child_scope(block as *mut luaur_ast::records::ast_node::AstNode, scope);
        let flow = self.visit_block_without_child_scope(
            inner_scope.as_ref() as *const Scope as *mut Scope,
            block,
        );

        // An AstStatBlock has linear control flow, i.e. one entry and one exit, so we
        // can inherit all the changes to the environment occurred by the statements in
        // that block.
        let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
        unsafe {
            (*scope_raw).inherit_assignments(&inner_scope);
            (*scope_raw).inherit_refinements(&inner_scope);
        }

        flow
    }
}
