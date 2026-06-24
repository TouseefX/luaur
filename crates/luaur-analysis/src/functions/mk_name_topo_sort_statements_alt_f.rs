use crate::records::identifier::Identifier;
use luaur_ast::records::ast_expr_error::AstExprError;

use luaur_common::functions::format::format;

pub fn mk_name_ast_expr_error(expr: &AstExprError) -> Identifier {
    Identifier::new(
        format(format_args!("error#{}", expr.message_index)),
        core::ptr::null(),
    )
}
