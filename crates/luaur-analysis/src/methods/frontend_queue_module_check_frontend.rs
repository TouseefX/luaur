use crate::records::frontend::Frontend;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use alloc::vec::Vec;

impl Frontend {
    pub fn queue_module_check_vector_module_name(&mut self, names: &Vec<ModuleName>) {
        self.module_queue.extend_from_slice(names);
    }
}
