use crate::records::frontend::Frontend;
use crate::type_aliases::module_name_file_resolver::ModuleName;

impl Frontend {
    pub fn all_module_dependencies_valid(&self, name: &ModuleName, for_autocomplete: bool) -> bool {
        if let Some(node) = self.source_nodes.get(name) {
            !node.has_invalid_module_dependency(for_autocomplete)
        } else {
            false
        }
    }
}
