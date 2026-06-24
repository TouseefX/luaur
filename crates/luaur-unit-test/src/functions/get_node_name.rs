use crate::records::test_require_node::TestRequireNode;
use alloc::string::String;

pub fn get_node_name(node: &TestRequireNode) -> String {
    let module_name = node.module_name.clone();
    if let Some(last_slash_pos) = module_name.rfind('/') {
        module_name[(last_slash_pos + 1)..].to_string()
    } else {
        module_name
    }
}
