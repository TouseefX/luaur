use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_declare_global(
        &mut self,
        decl_global: *mut AstStatDeclareGlobal,
    ) -> NonStrictContext {
        let type_ = unsafe { (*decl_global).type_ };
        self.visit_ast_type(type_);
        NonStrictContext::non_strict_context()
    }
}
