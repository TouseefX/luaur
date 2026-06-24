use crate::enums::type_context::TypeContext;
use crate::enums::value_context::ValueContext;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_if::AstStatIf;

impl TypeChecker2 {
    pub fn visit_ast_stat_if(&mut self, if_statement: *mut AstStatIf) {
        {
            // C++: `InConditionalContext flipper{&typeContext};` uses the default
            // `newValue = TypeContext::Condition` (TypeUtils.h:45). Visiting an if
            // statement's condition must set the conditional context so that
            // `inConditional(typeContext)` is true while checking the predicate.
            let _flipper =
                InConditionalContext::new(&mut self.type_context, TypeContext::Condition);
            unsafe {
                self.visit_ast_expr_value_context((*if_statement).condition, ValueContext::RValue);
            }
        }

        unsafe {
            self.visit_ast_stat_block((*if_statement).thenbody);
            if !(*if_statement).elsebody.is_null() {
                self.visit_ast_stat((*if_statement).elsebody);
            }
        }
    }
}
