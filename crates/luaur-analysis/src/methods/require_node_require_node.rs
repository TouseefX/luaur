use crate::records::require_node::RequireNode;

impl dyn RequireNode {
    // The C++ virtual destructor is handled by Rust's Box<dyn RequireNode> and trait object cleanup.
}
