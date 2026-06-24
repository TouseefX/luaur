use crate::records::lint_unbalanced_assignment::LintUnbalancedAssignment;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;
use luaur_ast::records::ast_expr_varargs::AstExprVarargs;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::location::Location;
use luaur_ast::rtti::ast_node_as;
use luaur_config::enums::code::Code;

impl LintUnbalancedAssignment {
    pub fn assign(&mut self, vars: usize, values: &AstArray<*mut AstExpr>, location: Location) {
        if vars != values.size && values.size > 0 {
            let last = unsafe { *values.data.add(values.size - 1) };

            if vars < values.size {
                let msg = format!(
                    "Assigning {} values to {} variables leaves some values unused",
                    values.size, vars
                );
                crate::functions::emit_warning::emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_UnbalancedAssignment,
                    location,
                    format_args!("{}", msg),
                );
            } else if !unsafe { ast_node_as::<AstExprCall>(last as *mut AstNode) }.is_null()
                || !unsafe { ast_node_as::<AstExprVarargs>(last as *mut AstNode) }.is_null()
                || !unsafe { ast_node_as::<AstExprConstantNil>(last as *mut AstNode) }.is_null()
            {
                // we don't know how many values the last expression returns
                // or last expression is nil which explicitly silences the nil-init warning
            } else {
                let msg = format!(
                    "Assigning {} values to {} variables initializes extra variables with nil; add 'nil' to value list to silence",
                    values.size, vars
                );
                crate::functions::emit_warning::emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_UnbalancedAssignment,
                    location,
                    format_args!("{}", msg),
                );
            }
        }
    }

    pub fn visit_stat_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = node as *mut AstStatLocal;
        unsafe {
            self.assign(
                (*node).vars.size,
                &(*node).values,
                (*node).base.base.location,
            );
        }

        true
    }

    pub fn visit_stat_assign(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = node as *mut AstStatAssign;
        unsafe {
            self.assign(
                (*node).vars.size,
                &(*node).values,
                (*node).base.base.location,
            );
        }

        true
    }
}
