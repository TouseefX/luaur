// ConstraintGenerator::visit(const ScopePtr&, AstStatRepeat*) (ConstraintGenerator.cpp:1707-1718).
use crate::enums::control_flow::ControlFlow;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::scope::Scope;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;

impl ConstraintGenerator {
    pub fn visit_scope_ptr_ast_stat_repeat(
        &mut self,
        scope: &ScopePtr,
        repeat: *mut AstStatRepeat,
    ) -> ControlFlow {
        let repeat_ref = unsafe { &*repeat };

        let repeat_scope: ScopePtr = self.child_scope(
            &repeat_ref.base.base as *const AstNode as *mut AstNode,
            scope,
        );

        self.visit_block_without_child_scope(
            repeat_scope.as_ref() as *const Scope as *mut Scope,
            repeat_ref.body,
        );

        self.check_scope_ptr_ast_expr(&repeat_scope, repeat_ref.condition);

        unsafe {
            (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_assignments(&repeat_scope);
        }

        ControlFlow::None
    }
}
