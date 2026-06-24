use crate::records::frontend_module_resolver::FrontendModuleResolver;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use crate::type_aliases::module_ptr_module_resolver::ModulePtr;

impl FrontendModuleResolver {
    /// C++ `FrontendModuleResolver::setModule` (`Analysis/src/Frontend.cpp:1970`):
    /// inserts/replaces under the module mutex, returning whether a prior entry
    /// was replaced.
    pub fn set_module(&mut self, module_name: &ModuleName, module: ModulePtr) -> bool {
        let _lock = self.module_mutex.lock().unwrap();

        let replaced = self.modules.contains_key(module_name);
        self.modules.insert(module_name.clone(), module);
        replaced
    }
}
