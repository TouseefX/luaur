use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_list::AstTypeList;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;

impl NonStrictTypeChecker {
    pub fn visit_ast_type_pack_explicit(&mut self, tp: *mut AstTypePackExplicit) {
        unsafe {
            let type_list = (*tp).type_list;
            let types = type_list.types;
            for i in 0..types.size {
                let ty = *types.data.add(i);
                self.visit_ast_type(ty);
            }

            let tail_type = type_list.tail_type;
            if !tail_type.is_null() {
                self.visit_ast_type_pack(tail_type as *mut AstTypePack);
            }
        }
    }
}
