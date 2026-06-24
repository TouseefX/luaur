use crate::records::global_name_collector::GlobalNameCollector;
use luaur_ast::records::ast_name::AstName;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl GlobalNameCollector {
    pub(crate) fn init_names(&mut self) {
        self.names = DenseHashSet::new(AstName {
            value: core::ptr::null(),
        });
    }
}
