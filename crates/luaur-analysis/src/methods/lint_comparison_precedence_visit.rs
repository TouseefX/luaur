use crate::functions::emit_warning::emit_warning;
use crate::records::lint_comparison_precedence::LintComparisonPrecedence;
use luaur_ast::functions::to_string_ast_alt_b::to_string_ast_expr_binary_op;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_expr_binary::AstExprBinary_Op;
use luaur_ast::rtti::ast_node_as;

impl LintComparisonPrecedence {
    pub fn visit_ast_expr_binary(&mut self, node: *mut AstExprBinary) -> bool {
        let op = unsafe { (*node).op };
        if !self.is_comparison(op) {
            return true;
        }

        let left = unsafe { (*node).left };
        let right = unsafe { (*node).right };

        let left_is_not = self.is_not(left);
        let right_is_not = self.is_not(right);

        if left_is_not && !right_is_not {
            let op_str = to_string_ast_expr_binary_op(op);
            let op_str_ref = op_str.as_str();

            if self.is_equality(op) {
                let opposite = if op == AstExprBinary_Op::CompareEq {
                    "~="
                } else {
                    "=="
                };
                emit_warning(
                    unsafe { &mut *self.context },
                    luaur_config::enums::code::Code::Code_ComparisonPrecedence,
                    unsafe { (*node).base.base.location },
                    format_args!(
                        "not X {} Y is equivalent to (not X) {} Y; consider using X {} Y, or add parentheses to silence",
                        op_str_ref, op_str_ref, opposite
                    ),
                );
            } else {
                emit_warning(
                    unsafe { &mut *self.context },
                    luaur_config::enums::code::Code::Code_ComparisonPrecedence,
                    unsafe { (*node).base.base.location },
                    format_args!(
                        "not X {} Y is equivalent to (not X) {} Y; add parentheses to silence",
                        op_str_ref, op_str_ref
                    ),
                );
            }
        } else {
            let left_binary = unsafe {
                ast_node_as::<AstExprBinary>(left as *mut luaur_ast::records::ast_node::AstNode)
            };
            if !left_binary.is_null() {
                let left_op = unsafe { (*left_binary).op };
                if self.is_comparison(left_op) {
                    let lop_str = to_string_ast_expr_binary_op(left_op);
                    let rop_str = to_string_ast_expr_binary_op(op);
                    let lop_str_ref = lop_str.as_str();
                    let rop_str_ref = rop_str.as_str();

                    if self.is_equality(left_op) || self.is_equality(op) {
                        emit_warning(
                            unsafe { &mut *self.context },
                            luaur_config::enums::code::Code::Code_ComparisonPrecedence,
                            unsafe { (*node).base.base.location },
                            format_args!(
                                "X {} Y {} Z is equivalent to (X {} Y) {} Z; add parentheses to silence",
                                lop_str_ref, rop_str_ref, lop_str_ref, rop_str_ref
                            ),
                        );
                    } else {
                        emit_warning(
                            unsafe { &mut *self.context },
                            luaur_config::enums::code::Code::Code_ComparisonPrecedence,
                            unsafe { (*node).base.base.location },
                            format_args!(
                                "X {} Y {} Z is equivalent to (X {} Y) {} Z; did you mean X {} Y and Y {} Z?",
                                lop_str_ref, rop_str_ref, lop_str_ref, rop_str_ref, lop_str_ref, rop_str_ref
                            ),
                        );
                    }
                }
            }
        }

        true
    }
}
