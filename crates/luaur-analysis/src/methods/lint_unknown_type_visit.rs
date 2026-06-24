use crate::enums::type_kind::TypeKind;
use crate::records::lint_unknown_type::LintUnknownType;
use core::ffi::CStr;
use core::mem::swap;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinary_Op};
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl LintUnknownType {
    pub fn visit(&mut self, node: *mut AstExprBinary) -> bool {
        let node_ref = unsafe { &*node };
        if node_ref.op != AstExprBinary_Op::CompareNe && node_ref.op != AstExprBinary_Op::CompareEq
        {
            return true;
        }

        let mut lhs = node_ref.left;
        let mut rhs = node_ref.right;

        // Ensure rhs is the constant string argument
        if !unsafe { (*rhs).base.is::<AstExprConstantString>() } {
            swap(&mut lhs, &mut rhs);
        }

        let call = unsafe { (*lhs).base.as_item::<AstExprCall>() };
        let arg = unsafe { (*rhs).base.as_item::<AstExprConstantString>() };

        if call.is_null() || arg.is_null() {
            return true;
        }

        let g = unsafe { (*(*call).func).base.as_item::<AstExprGlobal>() };
        if g.is_null() {
            return true;
        }

        let g_name = unsafe { CStr::from_ptr((*g).name.value) };
        if g_name.to_bytes() == b"type" {
            self.validate_type(
                arg as *mut AstExprConstantString,
                &[TypeKind::Kind_Primitive, TypeKind::Kind_Vector],
                "primitive type",
            );
        } else if g_name.to_bytes() == b"typeof" {
            self.validate_type(
                arg as *mut AstExprConstantString,
                &[TypeKind::Kind_Primitive, TypeKind::Kind_Userdata],
                "primitive or userdata type",
            );
        }

        true
    }
}
