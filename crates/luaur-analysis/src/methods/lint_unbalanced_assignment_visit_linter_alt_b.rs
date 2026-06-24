use crate::records::lint_unbalanced_assignment::LintUnbalancedAssignment;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

impl LintUnbalancedAssignment {
    pub fn visit_ast_stat_assign(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = node as *mut AstStatAssign;
        unsafe {
            self.assign(
                (*node).vars.size,
                &(*node).values,
                (*node).base.base.location,
            );
        }
        true
    }
}
