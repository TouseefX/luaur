use crate::records::expected_type_visitor::ExpectedTypeVisitor;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl ExpectedTypeVisitor {
    pub fn visit_ast_expr_type_assertion(&mut self, expr: *mut AstExprTypeAssertion) -> bool {
        let expr_ref = unsafe { &*expr };
        let ast_resolved_types = unsafe { &*self.ast_resolved_types };

        if let Some(annot) = ast_resolved_types.find(&(expr_ref.annotation as *const _)) {
            self.apply_expected_type(*annot, expr_ref.expr as *const _);
        }

        true
    }
}
