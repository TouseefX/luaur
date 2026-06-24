use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_compound_assign::AstStatCompoundAssign;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker2 {
    pub fn visit_ast_stat_compound_assign(&mut self, stat: *mut AstStatCompoundAssign) {
        unsafe {
            let location = (*stat).base.base.location;
            let op = (*stat).op;
            let var = (*stat).var;
            let value = (*stat).value;

            // C++: AstExprBinary fake{stat->location, stat->op, stat->var, stat->value}; visit(&fake, stat);
            let mut fake = AstExprBinary::new(location, op, var, value);
            self.visit_ast_expr_binary_ast_node(
                &mut fake as *mut AstExprBinary,
                stat as *mut luaur_ast::records::ast_node::AstNode,
            );

            let result_ty = (*self.module)
                .ast_compound_assign_result_types
                .find(&(stat as *const AstStat));

            if (*self.module).constraint_generation_did_not_complete && result_ty.is_none() {
                return;
            }

            LUAU_ASSERT!(result_ty.is_some());
            let result_ty = *result_ty.unwrap();
            let var_ty = self.lookup_type(var);

            self.test_is_subtype_type_id_type_id_location(result_ty, var_ty, location);
        }
    }
}
