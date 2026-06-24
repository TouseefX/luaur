use crate::records::file_resolver::FileResolver;
use crate::records::frontend_module_resolver::FrontendModuleResolver;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use alloc::string::String;

impl FrontendModuleResolver {
    pub fn get_human_readable_module_name(&self, module_name: &ModuleName) -> String {
        if self.frontend.is_null() {
            return module_name.clone();
        }

        unsafe {
            FileResolver::get_human_readable_module_name(
                (*self.frontend).file_resolver,
                module_name,
            )
        }
    }
}
