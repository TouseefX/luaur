use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_type_function::AstTypeFunction;

impl NonStrictTypeChecker {
    pub fn visit_ast_type_function(&mut self, function: *mut AstTypeFunction) {
        unsafe {
            self.visit_ast_type_list(&mut (*function).arg_types);
            self.visit_ast_type_pack((*function).return_types);
        }
    }
}
