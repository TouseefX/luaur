use crate::records::builtin_types::BuiltinTypes;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::type_arena::TypeArena;
use luaur_ast::records::ast_expr_binary::AstExprBinary;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_binary(&mut self, binary: *mut AstExprBinary) -> NonStrictContext {
        unsafe {
            let n = &*binary;
            let lhs = self.visit_ast_expr_value_context(
                n.left,
                crate::enums::value_context::ValueContext::RValue,
            );
            let rhs = self.visit_ast_expr_value_context(
                n.right,
                crate::enums::value_context::ValueContext::RValue,
            );
            let builtin_types = self.builtin_types;
            let arena = self.arena;
            NonStrictContext::disjunction(builtin_types, arena, &lhs, &rhs)
        }
    }
}
