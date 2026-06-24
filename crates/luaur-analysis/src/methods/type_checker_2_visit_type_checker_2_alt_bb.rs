use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_intersection::AstTypeIntersection;

impl TypeChecker2 {
    pub fn visit_ast_type_intersection(&mut self, ty: *mut AstTypeIntersection) {
        unsafe {
            let types = (*ty).types;
            for idx in 0..types.size {
                let ty = *types.data.add(idx);
                self.visit_ast_type(ty);
            }
        }
    }
}
