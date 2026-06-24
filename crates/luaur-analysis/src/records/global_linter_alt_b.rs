use alloc::vec::Vec;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct Global {
    pub(crate) firstRef: *mut AstExprGlobal,
    pub(crate) functionRef: Vec<*mut AstExprFunction>,
    pub(crate) assigned: bool,
    pub(crate) builtin: bool,
    pub(crate) definedInModuleScope: bool,
    pub(crate) definedAsFunction: bool,
    pub(crate) readBeforeWritten: bool,
    pub(crate) deprecated: Option<*const core::ffi::c_char>,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            firstRef: core::ptr::null_mut(),
            functionRef: Vec::new(),
            assigned: false,
            builtin: false,
            definedInModuleScope: false,
            definedAsFunction: false,
            readBeforeWritten: false,
            deprecated: None,
        }
    }
}

#[allow(non_snake_case)]
impl Global {
    pub fn firstRef(&self) -> *mut AstExprGlobal {
        self.firstRef
    }

    pub fn functionRef(&self) -> &[*mut AstExprFunction] {
        &self.functionRef
    }

    pub fn assigned(&self) -> bool {
        self.assigned
    }

    pub fn builtin(&self) -> bool {
        self.builtin
    }

    pub fn definedInModuleScope(&self) -> bool {
        self.definedInModuleScope
    }

    pub fn definedAsFunction(&self) -> bool {
        self.definedAsFunction
    }

    pub fn readBeforeWritten(&self) -> bool {
        self.readBeforeWritten
    }

    pub fn deprecated(&self) -> Option<*const core::ffi::c_char> {
        self.deprecated
    }
}
