use crate::functions::emit_warning::emit_warning;
use crate::functions::follow_type::follow_type_id;
use crate::records::lint_table_operations::LintTableOperations;
use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinary_Op};
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;
use luaur_ast::records::ast_node::AstNode;
use luaur_config::enums::code::Code;

impl LintTableOperations {
    pub fn check_table_call(&mut self, node: *mut AstExprCall, func: *mut AstExprIndexName) {
        let node_ref = unsafe { &*node };
        let args = node_ref.args.as_slice();

        if unsafe { (*func).index.operator_eq_c_char(c"insert".as_ptr()) }
            && node_ref.args.size == 2
        {
            let tail =
                unsafe { luaur_ast::rtti::ast_node_as::<AstExprCall>(args[1] as *mut AstNode) };

            if !tail.is_null() {
                if let Some(funty) = unsafe { (*self.context).get_type((*tail).func) } {
                    let ret = self.get_return_count(unsafe { follow_type_id(funty) });

                    if ret > 1 {
                        emit_warning(
                            unsafe { &mut *self.context },
                            Code::Code_TableOperations,
                            unsafe { (*tail).base.base.location },
                            format_args!(
                                "table.insert may change behavior if the call returns more than one result; consider adding parentheses around second argument"
                            ),
                        );
                    }
                }
            }
        }

        if unsafe { (*func).index.operator_eq_c_char(c"insert".as_ptr()) }
            && node_ref.args.size >= 3
        {
            if self.is_constant(args[1], 0.0) {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_TableOperations,
                    unsafe { (*args[1]).base.location },
                    format_args!(
                        "table.insert uses index 0 but arrays are 1-based; did you mean 1 instead?"
                    ),
                );
            }

            if self.is_length(args[1], args[0]) {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_TableOperations,
                    unsafe { (*args[1]).base.location },
                    format_args!(
                        "table.insert will insert the value before the last element, which is likely a bug; consider removing the second argument or wrap it in parentheses to silence"
                    ),
                );
            }

            let add =
                unsafe { luaur_ast::rtti::ast_node_as::<AstExprBinary>(args[1] as *mut AstNode) };
            if !add.is_null()
                && unsafe { (*add).op == AstExprBinary_Op::Add }
                && self.is_length(unsafe { (*add).left }, args[0])
                && self.is_constant(unsafe { (*add).right }, 1.0)
            {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_TableOperations,
                    unsafe { (*args[1]).base.location },
                    format_args!(
                        "table.insert will append the value to the table; consider removing the second argument for efficiency"
                    ),
                );
            }
        }

        if unsafe { (*func).index.operator_eq_c_char(c"remove".as_ptr()) }
            && node_ref.args.size >= 2
        {
            if self.is_constant(args[1], 0.0) {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_TableOperations,
                    unsafe { (*args[1]).base.location },
                    format_args!(
                        "table.remove uses index 0 but arrays are 1-based; did you mean 1 instead?"
                    ),
                );
            }

            let sub =
                unsafe { luaur_ast::rtti::ast_node_as::<AstExprBinary>(args[1] as *mut AstNode) };
            if !sub.is_null()
                && unsafe { (*sub).op == AstExprBinary_Op::Sub }
                && self.is_length(unsafe { (*sub).left }, args[0])
                && self.is_constant(unsafe { (*sub).right }, 1.0)
            {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_TableOperations,
                    unsafe { (*args[1]).base.location },
                    format_args!(
                        "table.remove will remove the value before the last element, which is likely a bug; consider removing the second argument or wrap it in parentheses to silence"
                    ),
                );
            }
        }

        if unsafe { (*func).index.operator_eq_c_char(c"move".as_ptr()) } && node_ref.args.size >= 4
        {
            if self.is_constant(args[1], 0.0) {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_TableOperations,
                    unsafe { (*args[1]).base.location },
                    format_args!(
                        "table.move uses index 0 but arrays are 1-based; did you mean 1 instead?"
                    ),
                );
            } else if self.is_constant(args[3], 0.0) {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_TableOperations,
                    unsafe { (*args[3]).base.location },
                    format_args!(
                        "table.move uses index 0 but arrays are 1-based; did you mean 1 instead?"
                    ),
                );
            }
        }

        if unsafe { (*func).index.operator_eq_c_char(c"create".as_ptr()) }
            && node_ref.args.size == 2
        {
            if !unsafe { luaur_ast::rtti::ast_node_as::<AstExprTable>(args[1] as *mut AstNode) }
                .is_null()
            {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_TableOperations,
                    unsafe { (*args[1]).base.location },
                    format_args!(
                        "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead"
                    ),
                );
            }

            let assertion = unsafe {
                luaur_ast::rtti::ast_node_as::<AstExprTypeAssertion>(args[1] as *mut AstNode)
            };
            if !assertion.is_null()
                && !unsafe {
                    luaur_ast::rtti::ast_node_as::<AstExprTable>((*assertion).expr as *mut AstNode)
                }
                .is_null()
            {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_TableOperations,
                    unsafe { (*(*assertion).expr).base.location },
                    format_args!(
                        "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead"
                    ),
                );
            }
        }
    }
}
