use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
use luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;

impl TypeChecker2 {
    pub fn visit_ast_type_pack(&mut self, pack: *mut AstTypePack) {
        if pack.is_null() {
            return;
        }

        unsafe {
            let node = pack as *mut AstNode;
            if (*node).is::<AstTypePackExplicit>() {
                self.visit_ast_type_pack_explicit(pack as *mut AstTypePackExplicit);
            } else if (*node).is::<AstTypePackVariadic>() {
                self.visit_ast_type_pack_variadic(pack as *mut AstTypePackVariadic);
            } else if (*node).is::<AstTypePackGeneric>() {
                self.visit_ast_type_pack_generic(pack as *mut AstTypePackGeneric);
            }
        }
    }
}
