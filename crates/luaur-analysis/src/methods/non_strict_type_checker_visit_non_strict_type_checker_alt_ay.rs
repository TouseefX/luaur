use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_type_union::AstTypeUnion;

impl NonStrictTypeChecker {
    pub fn visit_ast_type_union(&mut self, union_type: *mut AstTypeUnion) {
        unsafe {
            let types = (*union_type).types;
            for i in 0..types.size {
                let t = *types.data.add(i);
                self.visit_ast_type(t as *mut luaur_ast::records::ast_type::AstType);
            }
        }
    }
}
