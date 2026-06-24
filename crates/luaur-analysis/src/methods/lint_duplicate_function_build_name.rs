use crate::records::lint_duplicate_function::LintDuplicateFunction;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl LintDuplicateFunction {
    pub fn build_name(&self, expr: *mut AstExpr) -> String {
        unsafe {
            let local = ast_node_as::<AstExprLocal>(expr as *mut AstNode);
            if let Some(local) = local.as_ref() {
                let name = (*local.local).name;
                if !name.value.is_null() {
                    return core::ffi::CStr::from_ptr(name.value)
                        .to_string_lossy()
                        .into_owned();
                }
            }

            let global = ast_node_as::<AstExprGlobal>(expr as *mut AstNode);
            if let Some(global) = global.as_ref() {
                let name = global.name;
                if !name.value.is_null() {
                    return core::ffi::CStr::from_ptr(name.value)
                        .to_string_lossy()
                        .into_owned();
                }
            }

            let index_name = ast_node_as::<AstExprIndexName>(expr as *mut AstNode);
            if let Some(index_name) = index_name.as_ref() {
                let lhs = self.build_name(index_name.expr);
                if lhs.is_empty() {
                    return lhs;
                }
                let index = core::ffi::CStr::from_ptr(index_name.index.value).to_string_lossy();
                return format!("{}.{}", lhs, index);
            }
        }
        String::new()
    }
}
