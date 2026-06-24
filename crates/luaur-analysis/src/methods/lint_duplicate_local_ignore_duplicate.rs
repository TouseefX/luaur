use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_name::AstName;

pub fn lint_duplicate_local_ignore_duplicate(local: *mut AstLocal) -> bool {
    unsafe {
        (*local).name
            == AstName {
                value: b"_\0".as_ptr().cast(),
            }
    }
}
