use crate::records::file_resolver::FileResolver;
use crate::type_aliases::module_name_file_resolver::ModuleName;

impl FileResolver {
    pub fn get_environment_for_module_impl(
        &self,
        _name: &ModuleName,
    ) -> Option<alloc::string::String> {
        None
    }
}
