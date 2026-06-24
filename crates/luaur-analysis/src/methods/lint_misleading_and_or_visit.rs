use crate::records::lint_misleading_and_or::LintMisleadingAndOr;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_expr_binary::AstExprBinary_Op;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;
use luaur_ast::rtti::ast_node_as;

impl LintMisleadingAndOr {
    pub fn visit_ast_expr_binary(&mut self, node: *mut AstExprBinary) -> bool {
        if unsafe { (*node).op } != AstExprBinary_Op::Or {
            return true;
        }

        let left = unsafe { (*node).left };
        let and_ = unsafe {
            ast_node_as::<AstExprBinary>(left as *mut luaur_ast::records::ast_node::AstNode)
        };
        if and_.is_null() {
            return true;
        }

        if unsafe { (*and_).op } != AstExprBinary_Op::And {
            return true;
        }

        let mut alt: Option<&'static str> = None;

        let right = unsafe { (*and_).right };
        if !unsafe {
            ast_node_as::<AstExprConstantNil>(right as *mut luaur_ast::records::ast_node::AstNode)
        }
        .is_null()
        {
            alt = Some("nil");
        } else {
            let bool_node = unsafe {
                ast_node_as::<AstExprConstantBool>(
                    right as *mut luaur_ast::records::ast_node::AstNode,
                )
            };
            if !bool_node.is_null() && unsafe { (*bool_node).value } == false {
                alt = Some("false");
            }
        }

        if let Some(alt_val) = alt {
            crate::functions::emit_warning::emit_warning(
                unsafe { &mut *self.context },
                luaur_config::enums::code::Code::Code_MisleadingAndOr,
                unsafe { (*node).base.base.location },
                format_args!(
                    "The and-or expression always evaluates to the second alternative because the first alternative is {}; consider using if-then-else expression instead",
                    alt_val
                ),
            );
        }

        true
    }
}
