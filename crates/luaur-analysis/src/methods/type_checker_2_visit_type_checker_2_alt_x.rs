use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_class_method::AstClassMethod;
use luaur_ast::records::ast_class_property::AstClassProperty;
use luaur_ast::records::ast_stat_class::AstStatClass;
use luaur_common::FFlag;
use luaur_common::LUAU_ASSERT;

impl TypeChecker2 {
    pub fn visit_ast_stat_class(&mut self, stat: *mut AstStatClass) {
        LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());

        unsafe {
            let members = &(*stat).members;
            for i in 0..members.size as usize {
                let member = &*members.data.add(i);
                if let Some(prop) = member.get_if::<AstClassProperty>() {
                    if !prop.ty.is_null() {
                        self.visit_ast_type(prop.ty);
                    }
                } else if let Some(method) = member.get_if::<AstClassMethod>() {
                    self.visit_ast_expr_function(method.function);
                } else {
                    LUAU_ASSERT!(false);
                }
            }
        }
    }
}
