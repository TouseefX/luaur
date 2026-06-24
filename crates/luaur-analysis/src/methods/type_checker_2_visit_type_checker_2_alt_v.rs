use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;

impl TypeChecker2 {
    pub fn visit_ast_stat_declare_global(&mut self, stat: *mut AstStatDeclareGlobal) {
        unsafe {
            let type_ = (*stat).type_;
            self.visit_ast_type(type_);
        }
    }
}
