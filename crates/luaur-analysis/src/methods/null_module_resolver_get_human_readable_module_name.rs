use crate::records::null_module_resolver::NullModuleResolver;
use crate::type_aliases::module_name_type_fwd::ModuleName;

impl NullModuleResolver {
    pub fn get_human_readable_module_name(
        &self,
        module_name: &ModuleName,
    ) -> alloc::string::String {
        module_name.clone()
    }
}
