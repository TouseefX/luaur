use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_continue::AstStatContinue;

impl TypeChecker2 {
    pub fn visit_ast_stat_continue(&mut self, _stat: *mut AstStatContinue) {}
}
