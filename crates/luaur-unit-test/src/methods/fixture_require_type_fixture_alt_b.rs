use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_common::LUAU_ASSERT;

impl Fixture {
    pub fn require_type_module_name_string(&mut self, module_name: &str, name: &String) -> TypeId {
        let module = self
            .get_frontend()
            .module_resolver
            .get_module(&String::from(module_name));
        self.require_type_module_ptr_string(&module, name)
    }
}
