use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_type_union::AstTypeUnion;

impl TypeChecker2 {
    pub fn visit_ast_type_union(&mut self, ty: *mut AstTypeUnion) {
        unsafe {
            let types = (*ty).types;
            for i in 0..types.size {
                let t = *types.data.add(i);
                self.visit_ast_type(t as *mut luaur_ast::records::ast_type::AstType);
            }
        }
    }
}
