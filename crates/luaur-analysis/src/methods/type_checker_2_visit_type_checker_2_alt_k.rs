use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_stat_for::AstStatFor;

impl TypeChecker2 {
    pub fn visit_ast_stat_for(&mut self, for_statement: *mut AstStatFor) {
        unsafe {
            let for_ref = &*for_statement;

            if !(*for_ref.var).annotation.is_null() {
                let var_ref = &*for_ref.var;
                self.visit_ast_type(var_ref.annotation);
                let annotated_type = self.lookup_annotation(var_ref.annotation);
                self.test_is_subtype_type_id_type_id_location(
                    (*self.builtin_types).numberType,
                    annotated_type,
                    var_ref.location,
                );
            }

            // C++ uses a `checkNumber` lambda over [from, to, step]; inlined here.
            for &expr in &[for_ref.from, for_ref.to, for_ref.step] {
                self.check_number_for_stat(expr);
            }

            self.visit_ast_stat_block(for_ref.body);
        }
    }

    fn check_number_for_stat(&mut self, expr: *mut AstExpr) {
        if expr.is_null() {
            return;
        }
        unsafe {
            self.visit_ast_expr_value_context(expr, ValueContext::RValue);
            let expr_type = self.lookup_type(expr);
            self.test_is_subtype_type_id_type_id_location(
                expr_type,
                (*self.builtin_types).numberType,
                (*expr).base.location,
            );
        }
    }
}
