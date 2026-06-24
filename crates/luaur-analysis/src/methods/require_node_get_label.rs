use crate::records::require_node::RequireNode;
use alloc::string::String;

impl dyn RequireNode {
    pub fn get_label(&self) -> String {
        self.get_path_component()
    }
}
