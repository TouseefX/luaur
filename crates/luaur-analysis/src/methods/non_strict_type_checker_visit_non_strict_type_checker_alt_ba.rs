use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_list::AstTypeList;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl NonStrictTypeChecker {
    pub fn visit_ast_type_list(&mut self, list: &mut AstTypeList) {
        for i in 0..list.types.size {
            let t = unsafe { *list.types.data.add(i) };
            self.visit_ast_type(t as *mut AstType);
        }

        if !list.tail_type.is_null() {
            self.visit_ast_type_pack(list.tail_type as *mut AstTypePack);
        }
    }
}
