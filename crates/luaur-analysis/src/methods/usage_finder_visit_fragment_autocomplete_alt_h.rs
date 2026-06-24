use crate::records::usage_finder::UsageFinder;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::rtti::ast_node_as;

impl UsageFinder {
    pub fn visit_ast_stat_function(&mut self, function: *mut AstStatFunction) -> bool {
        let function_ref = unsafe { &*function };

        let name_expr = function_ref.name;
        if !name_expr.is_null() {
            let global = unsafe { ast_node_as::<AstExprGlobal>(name_expr as *mut AstNode) };

            if !global.is_null() {
                let global_name = unsafe { (*global).name };
                self.global_functions_referenced.push(global_name);
            }
        }

        true
    }
}
