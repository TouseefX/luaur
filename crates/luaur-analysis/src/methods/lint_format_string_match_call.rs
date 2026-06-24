use crate::functions::emit_warning::emit_warning;
use crate::functions::is_string::is_string;
use crate::records::lint_format_string::LintFormatString;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti;
use luaur_config::enums::code::Code;

impl LintFormatString {
    pub fn match_call(&mut self, node: *mut AstExprCall) {
        let node = unsafe { &*node };
        let func = unsafe { rtti::ast_node_as::<AstExprIndexName>(node.func as *mut AstNode) };
        if func.is_null() {
            return;
        }

        if node.self_ {
            let group = unsafe { rtti::ast_node_as::<AstExprGroup>((*func).expr as *mut AstNode) };
            let self_expr: *mut AstExpr = if !group.is_null() {
                unsafe { (*group).expr }
            } else {
                unsafe { (*func).expr }
            };

            if rtti::ast_node_is::<AstExprConstantString>(self_expr as *mut AstNode) {
                self.match_string_call(unsafe { (*func).index }, self_expr, node.args);
            } else if let Some(type_id) = unsafe { (*self.context).get_type(self_expr) } {
                if is_string(type_id) {
                    self.match_string_call(unsafe { (*func).index }, self_expr, node.args);
                }
            }
            return;
        }

        let lib = unsafe { rtti::ast_node_as::<AstExprGlobal>((*func).expr as *mut AstNode) };
        if lib.is_null() {
            return;
        }

        let lib_name = unsafe { (*lib).name };

        if lib_name.operator_eq_c_char(c"string".as_ptr()) {
            if node.args.size > 0 {
                let rest = AstArray {
                    data: unsafe { node.args.data.add(1) },
                    size: node.args.size - 1,
                };
                let first_arg = unsafe { *node.args.data.add(0) };
                self.match_string_call(unsafe { (*func).index }, first_arg, rest);
            }
        } else if lib_name.operator_eq_c_char(c"os".as_ptr()) {
            if unsafe { (*func).index }.operator_eq_c_char(c"date".as_ptr()) && node.args.size > 0 {
                let arg0 = unsafe { *node.args.data.add(0) };
                let fmt =
                    unsafe { rtti::ast_node_as::<AstExprConstantString>(arg0 as *mut AstNode) };
                if !fmt.is_null() {
                    let error = self.check_date_format(unsafe { (*fmt).value.data }, unsafe {
                        (*fmt).value.size
                    });
                    if !error.is_null() {
                        let error_str =
                            unsafe { core::ffi::CStr::from_ptr(error).to_string_lossy() };
                        emit_warning(
                            unsafe { &mut *self.context },
                            Code::Code_FormatString,
                            unsafe { (*fmt).base.base.location },
                            format_args!("Invalid date format: {}", error_str),
                        );
                    }
                }
            }
        }
    }
}
