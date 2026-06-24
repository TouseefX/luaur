use crate::functions::get_node_name::get_node_name;
use crate::records::test_require_node::TestRequireNode;
use alloc::string::String;

impl TestRequireNode {
    pub fn get_label(&self) -> String {
        get_node_name(self)
    }
}
