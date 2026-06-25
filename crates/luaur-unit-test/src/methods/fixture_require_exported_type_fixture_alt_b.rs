use crate::records::fixture::Fixture;
use alloc::string::{String, ToString};
use luaur_analysis::type_aliases::type_id::TypeId;

impl Fixture {
    /// C++ `TypeId Fixture::requireExportedType(const ModuleName& moduleName,
    /// const std::string& name)` (Fixture.cpp): resolve the module, then return
    /// the underlying type of the named exported type binding.
    pub fn require_exported_type_module_name_string(
        &mut self,
        module_name: &str,
        name: &String,
    ) -> TypeId {
        let module = self
            .get_frontend()
            .module_resolver
            .get_module(&module_name.to_string());
        let binding = module
            .exported_type_bindings
            .get(name)
            .expect("exported type binding not found");
        binding.r#type()
    }
}
