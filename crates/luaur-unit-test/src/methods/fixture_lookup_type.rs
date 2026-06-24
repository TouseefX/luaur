use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::type_aliases::type_id::TypeId;

impl Fixture {
    pub fn lookup_type(&mut self, name: &String) -> Option<TypeId> {
        let module = self.get_main_module(false);
        if module.is_null() {
            panic!("lookupType: No main module");
        }

        let module = unsafe { &*module };
        if !module.has_module_scope() {
            return None;
        }

        let scope = module.get_module_scope();
        let type_fun = scope.lookup_type(name)?;
        Some(type_fun.r#type())
    }
}
