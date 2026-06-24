use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::type_aliases::type_id::TypeId;

impl Fixture {
    pub fn lookup_imported_type(&mut self, module_alias: &String, name: &String) -> Option<TypeId> {
        let module = self.get_main_module(false);
        let module = unsafe { &*module };

        if !module.has_module_scope() {
            panic!("lookupImportedType: module scope data is not available");
        }

        let scope = module.get_module_scope();
        if let Some(type_fun) = scope.lookup_imported_type(module_alias, name) {
            return Some(type_fun.r#type());
        }

        None
    }
}
