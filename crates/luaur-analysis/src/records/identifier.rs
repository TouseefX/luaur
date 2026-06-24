use alloc::string::String;
use luaur_ast::records::ast_local::AstLocal;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub(crate) name: String,
    pub(crate) ctx: *const AstLocal,
}

impl Identifier {
    pub fn new(name: String, ctx: *const AstLocal) -> Self {
        Self { name, ctx }
    }
}

#[allow(non_snake_case)]
impl Identifier {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ctx(&self) -> *const AstLocal {
        self.ctx
    }
}
