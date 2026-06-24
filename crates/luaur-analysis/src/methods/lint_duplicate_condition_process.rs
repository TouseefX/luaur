use crate::records::lint_duplicate_condition::LintDuplicateCondition;
use luaur_ast::visit::ast_stat_visit;

impl LintDuplicateCondition {
    pub fn process(&mut self) {
        unsafe {
            ast_stat_visit((*self.context).root, self);
        }
    }
}
