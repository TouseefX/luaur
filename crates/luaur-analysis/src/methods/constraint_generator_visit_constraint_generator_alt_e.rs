// ConstraintGenerator::visit(const ScopePtr&, AstStatWhile*) (ConstraintGenerator.cpp:1693-1705).
use crate::enums::control_flow::ControlFlow;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::scope::Scope;
use crate::type_aliases::refinement_id_refinement::RefinementId;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_while::AstStatWhile;

impl ConstraintGenerator {
    pub fn visit_scope_ptr_ast_stat_while(
        &mut self,
        scope: &ScopePtr,
        while_: *mut AstStatWhile,
    ) -> ControlFlow {
        let while_ref = unsafe { &*while_ };

        let refinement: RefinementId = self
            .check_scope_ptr_ast_expr(scope, while_ref.condition)
            .refinement;

        let while_scope: ScopePtr = self.child_scope(
            &while_ref.base.base as *const AstNode as *mut AstNode,
            scope,
        );
        self.apply_refinements(
            &while_scope,
            unsafe { (*while_ref.condition).base.location },
            refinement,
        );

        self.visit_scope_ptr_ast_stat_block(&while_scope, while_ref.body);

        unsafe {
            (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_assignments(&while_scope);
        }

        ControlFlow::None
    }
}
