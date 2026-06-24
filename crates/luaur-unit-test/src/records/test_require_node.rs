use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TestRequireNode {
    pub(crate) module_name: ModuleName,
    pub(crate) all_sources: *const HashMap<ModuleName, alloc::string::String>,
}
