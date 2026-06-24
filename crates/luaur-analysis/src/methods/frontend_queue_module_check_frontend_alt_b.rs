use crate::records::frontend::Frontend;
use crate::type_aliases::module_name_file_resolver::ModuleName;

impl Frontend {
    pub fn queue_module_check_module_name(&mut self, _name: &ModuleName) {
        self.queue_module_check_vector_module_name(&vec![_name.clone()]);
    }
}
