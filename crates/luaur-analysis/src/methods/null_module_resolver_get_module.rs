use crate::records::null_module_resolver::NullModuleResolver;
use crate::type_aliases::module_name_type_fwd::ModuleName;
use crate::type_aliases::module_ptr_module_resolver::ModulePtr;

impl NullModuleResolver {
    /// C++ `const ModulePtr getModule(const ModuleName&) const override { return nullptr; }`
    /// (ModuleResolver.h:57). The nullable `ModulePtr` is modeled as
    /// `Option<ModulePtr>`, so the null answer is `None`.
    pub fn get_module(&self, _module_name: &ModuleName) -> Option<ModulePtr> {
        None
    }
}
