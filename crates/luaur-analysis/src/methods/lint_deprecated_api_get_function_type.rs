use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use luaur_ast::records::ast_expr::AstExpr;

impl LintDeprecatedApi {
    pub fn get_function_type(&self, node: *mut AstExpr) -> *const FunctionType {
        let ty = unsafe { (*self.context).get_type(node) };
        if let Some(t) = ty {
            let followed = unsafe { follow_type_id(t) };
            unsafe { get_type_id::<FunctionType>(followed) }
        } else {
            core::ptr::null()
        }
    }
}
