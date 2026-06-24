use crate::records::require_node::RequireNode;
use alloc::string::String;
use alloc::vec::Vec;

impl dyn RequireNode {
    pub fn get_tags(&self) -> Vec<String> {
        Vec::new()
    }
}
