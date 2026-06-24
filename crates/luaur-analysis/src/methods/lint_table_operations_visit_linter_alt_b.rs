//! C++ `LintTableOperations::visit(AstExprCall*)` (`Analysis/src/Linter.cpp:2618`).

use crate::records::lint_table_operations::LintTableOperations;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;

impl LintTableOperations {
    pub fn visit_ast_expr_call(&mut self, node: *mut AstExprCall) -> bool {
        unsafe {
            let func_expr = (*node).func;
            let func_global =
                luaur_ast::rtti::ast_node_as::<AstExprGlobal>(func_expr as *mut AstNode);

            if !func_global.is_null() {
                if (*func_global).name.operator_eq_c_char(c"ipairs".as_ptr())
                    && (*node).args.size == 1
                {
                    let arg0 = *(*node).args.data.add(0);
                    self.check_indexer(node as *mut AstExpr, arg0, "ipairs");
                }
            } else {
                let func_index =
                    luaur_ast::rtti::ast_node_as::<AstExprIndexName>(func_expr as *mut AstNode);
                if !func_index.is_null() {
                    let tablib = luaur_ast::rtti::ast_node_as::<AstExprGlobal>(
                        (*func_index).expr as *mut AstNode,
                    );
                    if !tablib.is_null() && (*tablib).name.operator_eq_c_char(c"table".as_ptr()) {
                        self.check_table_call(node, func_index);
                    }
                }
            }
        }

        true
    }
}
