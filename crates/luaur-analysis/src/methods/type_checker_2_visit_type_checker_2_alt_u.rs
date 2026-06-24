use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction;

impl TypeChecker2 {
    pub fn visit_ast_stat_declare_function(&mut self, stat: *mut AstStatDeclareFunction) {
        unsafe {
            let generics = (*stat).generics;
            let generic_packs = (*stat).generic_packs;
            let params = (*stat).params;
            let ret_types = (*stat).ret_types;

            self.visit_generics(generics, generic_packs);
            self.visit_ast_type_list(&mut params.clone());
            self.visit_ast_type_pack(ret_types);
        }
    }
}
