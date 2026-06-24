use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_stat_continue::AstStatContinue;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_continue(&mut self, _continue_statement: *mut AstStatContinue) {
        let _ = NonStrictContext::non_strict_context();
    }
}
