use crate::records::fixture::Fixture;
use alloc::string::String;
use alloc::sync::Arc;
use luaur_analysis::records::module::Module;
use luaur_analysis::type_aliases::type_id::TypeId;

impl Fixture {
    pub fn require_type_module_ptr_string(&mut self, module: &Module, name: &String) -> TypeId {
        let scope = module.get_module_scope();
        let scope = Arc::as_ptr(&scope) as *mut _;
        self.require_type_scope_ptr_string(scope, name)
    }
}
