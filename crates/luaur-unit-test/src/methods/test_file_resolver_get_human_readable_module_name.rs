use crate::records::test_file_resolver::TestFileResolver;
use alloc::string::String;

impl TestFileResolver {
    pub fn get_human_readable_module_name(&self, name: &str) -> String {
        name.replace('/', ".")
    }
}
