use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::type_aliases::type_id::TypeId;

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
    pub fn require_exported_type_string(&mut self, name: &String) -> TypeId {
        self.require_exported_type_module_name_string(MAIN_MODULE_NAME, name)
    }
}
