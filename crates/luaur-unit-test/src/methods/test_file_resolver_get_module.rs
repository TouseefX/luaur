use crate::records::test_file_resolver::TestFileResolver;
use luaur_analysis::records::module::Module;

impl TestFileResolver {
    pub fn get_module(&self, module_name: &str) -> *mut Module {
        let _ = module_name;
        core::ptr::null_mut()
    }
}
