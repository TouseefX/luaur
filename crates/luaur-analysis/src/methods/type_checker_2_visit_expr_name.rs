use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn visit_expr_name(
        &mut self,
        expr: *mut AstExpr,
        location: Location,
        prop_name: &str,
        context: ValueContext,
        ast_index_expr_ty: TypeId,
    ) {
        self.visit_ast_expr_value_context(expr, ValueContext::RValue);
        let inferred = self.lookup_type(expr);
        let left_type = self.strip_from_nil_and_report(inferred, &location);
        self.check_index_type_from_type(left_type, prop_name, context, location, ast_index_expr_ty);
    }
}
