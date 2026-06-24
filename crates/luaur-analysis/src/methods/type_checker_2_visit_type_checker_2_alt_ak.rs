use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

impl TypeChecker2 {
    pub fn visit_ast_expr_index_name_value_context(
        &mut self,
        index_name: *mut AstExprIndexName,
        context: ValueContext,
    ) {
        unsafe {
            let expr = (*index_name).expr;
            let location = (*index_name).base.base.location;
            let index = (*index_name).index;
            let prop_name = core::ffi::CStr::from_ptr(index.value).to_string_lossy();
            let ast_index_expr_ty = (*self.builtin_types).stringType;
            self.visit_expr_name(expr, location, &prop_name, context, ast_index_expr_ty);
        }
    }
}
