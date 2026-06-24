use crate::records::lint_deprecated_api::LintDeprecatedApi;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl LintDeprecatedApi {
    pub fn visit_ast_expr_index_name(&mut self, node: *mut AstExprIndexName) -> bool {
        unsafe {
            if let Some(ty) = (*self.context).get_type((*node).expr) {
                self.check_ast_expr_index_name_type_id(node, ty);
            } else {
                let global = ast_node_as::<AstExprGlobal>((*node).expr as *mut AstNode);
                if let Some(global) = global.as_ref() {
                    self.check_location_ast_name_ast_name(
                        &(*node).base.base.location,
                        global.name,
                        (*node).index,
                    );
                }
            }
        }

        true
    }
}
