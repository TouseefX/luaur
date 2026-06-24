use crate::records::symbol::Symbol;
use luaur_ast::records::ast_name::AstName;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Symbol {
    pub fn ast_name(&self) -> AstName {
        if !self.local.is_null() {
            return unsafe { (*self.local).name };
        }

        LUAU_ASSERT!(!self.global.value.is_null());
        self.global
    }
}
