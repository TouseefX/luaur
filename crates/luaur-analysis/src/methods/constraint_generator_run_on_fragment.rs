use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::constraint_set::ConstraintSet;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_stat_block::AstStatBlock;

impl ConstraintGenerator {
    pub fn run_on_fragment(
        &mut self,
        resume_scope: &ScopePtr,
        block: *mut AstStatBlock,
    ) -> ConstraintSet {
        self.visit_fragment_root(resume_scope, block);

        ConstraintSet {
            root_scope: self.root_scope,
            constraints: self.constraints.clone(),
            free_types: self.free_types.clone(),
            scope_to_function: self.scope_to_function.clone(),
            errors: self.errors.clone(),
        }
    }
}
