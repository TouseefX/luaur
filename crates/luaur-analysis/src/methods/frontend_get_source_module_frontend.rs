use crate::records::frontend::Frontend;
use crate::records::source_module::SourceModule;
use crate::type_aliases::module_name_file_resolver::ModuleName;

impl Frontend {
    pub fn get_source_module_mut(&mut self, module_name: &ModuleName) -> *mut SourceModule {
        if let Some(source_module) = self.source_modules.get(module_name) {
            let ptr = source_module.as_ref() as *const SourceModule;
            ptr as *mut SourceModule
        } else {
            core::ptr::null_mut()
        }
    }
}
