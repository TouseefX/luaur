//! Faithful port of `Luau::detail::ArcCollector::ArcCollector`
//! (`Analysis/src/TopoSortStatements.cpp:207-217`).
//!
//! ```cpp
//! ArcCollector(NodeQueue& queue)
//!     : queue(queue)
//!     , map(Identifier{std::string{}, 0})
//!     , currentArc(nullptr)
//! {
//!     for (const auto& node : queue)
//!     {
//!         if (node->name && !map.contains(*node->name))
//!             map[*node->name] = node.get();
//!     }
//! }
//! ```
use crate::records::arc_collector::ArcCollector;
use crate::records::identifier::Identifier;
use crate::records::identifier_hash::IdentifierHash;
use crate::records::node::Node;
use crate::type_aliases::node_queue::NodeQueue;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_table::DenseHasher;

// Wires the C++ `IdentifierHash::operator()` functor (used as the `Hash`
// template parameter of `DenseHashMap<Identifier, Node*, IdentifierHash>`) into
// the container's `DenseHasher` trait so the map is constructible. The hash body
// itself already lives in `IdentifierHash::identifier_hash_operator_call`.
impl DenseHasher<Identifier> for IdentifierHash {
    fn hash(&self, key: &Identifier) -> usize {
        IdentifierHash::identifier_hash_operator_call(key)
    }
}

impl ArcCollector {
    pub fn arc_collector(&mut self, queue: &mut NodeQueue) {
        // : queue(queue), map(Identifier{std::string{}, 0}), currentArc(nullptr)
        self.queue = queue as *mut NodeQueue;
        self.map = DenseHashMap::new(Identifier::new(
            alloc::string::String::new(),
            core::ptr::null(),
        ));
        self.current_arc = core::ptr::null_mut();

        // for (const auto& node : queue)
        //     if (node->name && !map.contains(*node->name))
        //         map[*node->name] = node.get();
        let size = queue.size();
        for i in 0..size {
            let node_ptr: *mut Node = *queue.at(i);
            let node = unsafe { &*node_ptr };
            if let Some(name) = &node.name {
                if !self.map.contains(name) {
                    *self.map.get_or_insert(name.clone()) = node_ptr;
                }
            }
        }
    }
}
