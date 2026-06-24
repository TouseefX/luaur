use crate::records::global_linter::Global;
use luaur_ast::records::ast_name::AstName;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_table::DenseDefault;

impl DenseDefault for Global {
    fn dense_default() -> Self {
        unsafe { core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

impl crate::records::lint_context::LintContext {
    pub fn lint_context(&mut self) {
        self.root = core::ptr::null_mut();
        self.placeholder = AstName::new();
        self.builtin_globals = DenseHashMap::new(AstName::new());
        self.module = core::ptr::null();
    }
}
