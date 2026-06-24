use crate::records::frontend_module_resolver::FrontendModuleResolver;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use crate::type_aliases::module_ptr_module_resolver::ModulePtr;

impl FrontendModuleResolver {
    pub fn get_module(&self, module_name: &ModuleName) -> ModulePtr {
        let _lock = self.module_mutex.lock().unwrap();
        self.modules
            .get(module_name)
            .cloned()
            .unwrap_or_else(|| panic!("Frontend does not have module: {}", module_name))
    }
}
