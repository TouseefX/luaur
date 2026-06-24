use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_type_list::AstTypeList;

impl TypeChecker2 {
    pub fn visit_ast_type_list_mut(&mut self, type_list: *mut AstTypeList) {
        unsafe {
            let types = (*type_list).types;
            for i in 0..types.size {
                let ty = *types.data.add(i);
                self.visit_ast_type(ty);
            }
            let tail_type = (*type_list).tail_type;
            if !tail_type.is_null() {
                self.visit_ast_type_pack(tail_type);
            }
        }
    }
}
