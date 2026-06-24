use crate::enums::type_kind::TypeKind;
use crate::functions::emit_warning::emit_warning;
use crate::records::lint_unknown_type::LintUnknownType;
use alloc::string::String;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_config::enums::code::Code;

impl LintUnknownType {
    pub fn validate_type(
        &mut self,
        expr: *mut AstExprConstantString,
        expected: &[TypeKind],
        expected_string: &str,
    ) {
        let expr = unsafe { &*expr };
        let name_bytes = unsafe {
            core::slice::from_raw_parts(expr.value.data as *const u8, expr.value.size as usize)
        };
        let name = String::from_utf8_lossy(name_bytes);
        let kind = self.get_type_kind(&name);

        if kind == TypeKind::Kind_Unknown {
            let msg = format!("Unknown type '{}'", name);
            emit_warning(
                unsafe { &mut *self.context },
                Code::Code_UnknownType,
                expr.base.base.location,
                format_args!("{}", msg),
            );
            return;
        }

        for &ek in expected {
            if kind == ek {
                return;
            }
        }

        let msg = format!("Unknown type '{}' (expected {})", name, expected_string);
        emit_warning(
            unsafe { &mut *self.context },
            Code::Code_UnknownType,
            expr.base.base.location,
            format_args!("{}", msg),
        );
    }
}
