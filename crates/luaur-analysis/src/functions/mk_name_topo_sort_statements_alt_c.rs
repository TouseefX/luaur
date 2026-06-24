use crate::records::identifier::Identifier;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

pub fn mk_name_ast_expr_global(global: &AstExprGlobal) -> Identifier {
    Identifier::new(
        unsafe {
            std::ffi::CStr::from_ptr(global.name.value)
                .to_string_lossy()
                .into_owned()
        },
        std::ptr::null(),
    )
}
