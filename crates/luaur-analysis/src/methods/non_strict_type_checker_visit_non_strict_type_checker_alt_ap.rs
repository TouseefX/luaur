use crate::records::builtin_types::BuiltinTypes;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::type_arena::TypeArena;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_if_else(&mut self, if_else: *mut AstExprIfElse) -> NonStrictContext {
        let cond_b = unsafe {
            self.visit_ast_expr_value_context(
                (*if_else).condition,
                crate::enums::value_context::ValueContext::RValue,
            )
        };
        let then_b = unsafe {
            self.visit_ast_expr_value_context(
                (*if_else).true_expr,
                crate::enums::value_context::ValueContext::RValue,
            )
        };
        let else_b = unsafe {
            self.visit_ast_expr_value_context(
                (*if_else).false_expr,
                crate::enums::value_context::ValueContext::RValue,
            )
        };

        NonStrictContext::conjunction(self.builtin_types, self.arena, &then_b, &else_b)
    }
}
