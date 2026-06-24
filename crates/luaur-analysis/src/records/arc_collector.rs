use crate::records::identifier::Identifier;
use crate::records::identifier_hash::IdentifierHash;
use crate::records::node::Node;
use crate::type_aliases::node_queue::NodeQueue;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct ArcCollector {
    pub queue: *mut NodeQueue,
    pub map: DenseHashMap<Identifier, *mut Node, IdentifierHash>,
    pub current_arc: *mut Node,
}
