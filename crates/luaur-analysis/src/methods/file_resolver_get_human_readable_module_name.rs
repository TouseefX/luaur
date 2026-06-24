use crate::records::file_resolver::FileResolver;
use crate::type_aliases::module_name_file_resolver::ModuleName;

impl FileResolver {
    pub fn get_human_readable_module_name_impl(&self, name: &ModuleName) -> alloc::string::String {
        name.clone()
    }
}
