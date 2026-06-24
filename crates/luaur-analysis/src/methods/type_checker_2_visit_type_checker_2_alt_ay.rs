use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_type_function::AstTypeFunction;
use luaur_ast::records::ast_type_list::AstTypeList;

impl TypeChecker2 {
    pub fn visit_ast_type_function(&mut self, ty: *mut AstTypeFunction) {
        unsafe {
            let generics = (*ty).generics;
            let generic_packs = (*ty).generic_packs;
            let return_types = (*ty).return_types;

            self.visit_generics(generics, generic_packs);
            self.visit_ast_type_list(&mut (*ty).arg_types as *mut AstTypeList);
            self.visit_ast_type_pack(return_types);
        }
    }
}
