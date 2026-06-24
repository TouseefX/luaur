use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_intersection::AstTypeIntersection;

impl NonStrictTypeChecker {
    pub fn visit_ast_type_intersection(&mut self, intersection_type: *mut AstTypeIntersection) {
        unsafe {
            let types = (*intersection_type).types;
            for idx in 0..types.size {
                let ty = *types.data.add(idx);
                self.visit_ast_type(ty as *mut AstType);
            }
        }
    }
}
