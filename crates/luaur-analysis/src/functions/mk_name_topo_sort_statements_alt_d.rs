use crate::records::identifier::Identifier;
use luaur_ast::records::ast_name::AstName;

pub fn mk_name_ast_name(name: &AstName) -> Identifier {
    Identifier::new(
        unsafe {
            core::ffi::CStr::from_ptr(name.value)
                .to_string_lossy()
                .into_owned()
        },
        core::ptr::null(),
    )
}
