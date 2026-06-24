use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_stat_class::AstStatClass;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_class(&mut self, decl_class: *mut AstStatClass) -> NonStrictContext {
        unsafe {
            let members = &(*decl_class).members;
            for i in 0..members.size {
                let prop = &*members.data.add(i);
                if let Some(property) = prop.get_if_0() {
                    self.visit_ast_type(property.ty);
                } else if let Some(method) = prop.get_if_1() {
                    self.visit_ast_expr_function(method.function);
                } else {
                    LUAU_ASSERT!(false);
                }
            }

            NonStrictContext::non_strict_context()
        }
    }
}
