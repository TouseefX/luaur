use crate::records::null_module_resolver::NullModuleResolver;
use crate::type_aliases::module_name_type_fwd::ModuleName;

impl NullModuleResolver {
    pub fn module_exists(&self, _module_name: &ModuleName) -> bool {
        false
    }
}
