use alloc::string::String;
use luaur_ast::records::ast_expr::AstExpr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeGuard {
    pub(crate) is_typeof: bool,
    pub(crate) target: *mut AstExpr,
    pub(crate) r#type: String,
}

impl Default for TypeGuard {
    fn default() -> Self {
        Self {
            is_typeof: false,
            target: core::ptr::null_mut(),
            r#type: String::new(),
        }
    }
}

#[allow(non_snake_case)]
impl TypeGuard {
    pub fn isTypeof(&self) -> bool {
        self.is_typeof
    }

    pub fn target(&self) -> *mut AstExpr {
        self.target
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }
}
