use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_stat_break::AstStatBreak;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_break(&mut self, _break_statement: *mut AstStatBreak) {}
}
