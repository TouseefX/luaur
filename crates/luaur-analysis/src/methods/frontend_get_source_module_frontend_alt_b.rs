use crate::records::frontend::Frontend;
use crate::records::source_module::SourceModule;
use crate::type_aliases::module_name_type_fwd::ModuleName;

impl Frontend {
    pub fn get_source_module(&self, module_name: &ModuleName) -> *const SourceModule {
        let this_mut = self as *const Frontend as *mut Frontend;
        unsafe { (*this_mut).get_source_module_mut(module_name) as *const SourceModule }
    }
}
