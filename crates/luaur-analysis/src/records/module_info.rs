use crate::type_aliases::module_name_file_resolver::ModuleName;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleInfo {
    pub name: ModuleName,
    pub optional: bool,
}

impl Default for ModuleInfo {
    fn default() -> Self {
        Self {
            name: ModuleName::default(),
            optional: false,
        }
    }
}
