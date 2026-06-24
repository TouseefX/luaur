use crate::records::lint_context::LintContext;
use crate::records::lint_table_operations::LintTableOperations;

impl LintTableOperations {
    pub fn lint_table_operations(context: *mut LintContext) -> Self {
        LintTableOperations { context }
    }
}
