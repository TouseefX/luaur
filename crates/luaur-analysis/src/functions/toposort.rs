//! Faithful port of `Luau::toposort`
//! (`Analysis/src/TopoSortStatements.cpp:520-583`).
//!
//! Decide the order in which to typecheck a block of statements: build a
//! dependency graph (uses → declarations, plus a chain through imperative
//! statements), then walk it Kahn-style, deferring function/type definitions
//! into a queue `Q` that is toposorted on demand.
use crate::functions::contains_function_call::contains_function_call;
use crate::functions::drain::drain;
use crate::functions::is_block_terminator::is_block_terminator;
use crate::functions::is_toposortable_node::is_toposortable_node;
use crate::functions::mk_name_topo_sort_statements_alt_m::mk_name_ast_stat;
use crate::functions::prune::prune;
use crate::records::arc_collector::ArcCollector;
use crate::records::identifier::Identifier;
use crate::records::identifier_hash::IdentifierHash;
use crate::records::node::Node;
use crate::type_aliases::node_list::NodeList;
use crate::type_aliases::node_queue::NodeQueue;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_common::records::dense_hash_map::DenseHashMap;

// Wire `ArcCollector` (a C++ `AstVisitor` subclass) into the Rust `AstVisitor`
// trait so `ast_stat_visit` dispatches to its overrides. Each trait method just
// forwards to the inherent `visit_ast_*` method that carries the ported body;
// the un-overridden methods keep the base `AstVisitor` defaults — exactly the
// set the C++ class overrides (the `AstExpr*`/`AstStat*`/`AstType*` ones).
impl AstVisitor for ArcCollector {
    fn visit_expr_global(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_global(node)
    }
    fn visit_expr_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_local(node)
    }
    fn visit_expr_index_name(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_index_name(node)
    }
    fn visit_stat_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_function(node)
    }
    fn visit_stat_local_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_local_function(node)
    }
    fn visit_stat_assign(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_assign(node)
    }
    fn visit_stat_type_alias(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_type_alias(node)
    }
    fn visit_type(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type(node)
    }
    fn visit_type_reference(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_reference(node)
    }
    fn visit_type_typeof(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_typeof(node)
    }
    fn visit_type_pack(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_pack(node)
    }
}

pub fn toposort(stats: &mut Vec<*mut AstStat>) {
    // if (stats.empty()) return;
    if stats.is_empty() {
        return;
    }

    // if (!containsToposortableNode(stats)) return;
    // (Inlined: the helper's signature does not match `Vec<*mut AstStat>`.)
    if !stats
        .iter()
        .any(|&stat| is_toposortable_node(unsafe { &*stat }))
    {
        return;
    }

    // std::vector<AstStat*> result; result.reserve(stats.size());
    let mut result: Vec<*mut AstStat> = Vec::with_capacity(stats.len());

    // NodeQueue nodes; NodeList Q;
    let mut nodes: NodeQueue = NodeQueue::new();
    let mut q: NodeList = NodeList::new();

    // Owns every allocated Node so it can be freed once at the end (the C++
    // `unique_ptr`s owned by `nodes`/`Q` free automatically at scope exit).
    let mut all_nodes: Vec<*mut Node> = Vec::with_capacity(stats.len());

    // for (AstStat* stat : stats) nodes.push_back(new Node(mkName(stat), stat));
    for &stat in stats.iter() {
        let node_ptr = alloc::boxed::Box::into_raw(alloc::boxed::Box::new(Node::new(
            mk_name_ast_stat(stat),
            stat,
        )));
        all_nodes.push(node_ptr);
        nodes.push_back(node_ptr);
    }

    // ArcCollector collector{nodes};
    let mut collector = ArcCollector {
        queue: core::ptr::null_mut(),
        map: DenseHashMap::<Identifier, *mut Node, IdentifierHash>::new(Identifier::new(
            alloc::string::String::new(),
            core::ptr::null(),
        )),
        current_arc: core::ptr::null_mut(),
    };
    collector.arc_collector(&mut nodes);

    // for (const auto& node : nodes) { collector.currentArc = node.get(); node->element->visit(&collector); }
    {
        let count = nodes.size();
        for i in 0..count {
            let node_ptr: *mut Node = *nodes.at(i);
            collector.current_arc = node_ptr;
            let element = unsafe { (*node_ptr).element };
            unsafe {
                luaur_ast::visit::ast_stat_visit(element, &mut collector);
            }
        }
    }

    // Prev-statement edges: chain each non-toposortable statement to the
    // previous non-toposortable statement.
    {
        let count = nodes.size();
        let mut prev: usize = 0;
        let mut it: usize = 0;
        while it < count {
            let it_ptr: *mut Node = *nodes.at(it);
            if it != prev && !is_toposortable_node(unsafe { &*(*it_ptr).element }) {
                let prev_ptr: *mut Node = *nodes.at(prev);
                unsafe {
                    (*it_ptr).depends.insert(prev_ptr);
                    (*prev_ptr).provides.insert(it_ptr);
                }
                prev = it;
            }
            it += 1;
        }
    }

    // while (!nodes.empty()) { ... }
    while !nodes.empty() {
        let next: *mut Node = *nodes.front();

        if unsafe { (*next).depends.is_empty() }
            && !is_block_terminator(unsafe { &*(*next).element })
        {
            prune(next);
            result.push(unsafe { (*next).element });
        } else if !contains_function_call(unsafe { &*(*next).element }) {
            q.push_back(next);
        } else {
            drain(&mut q, &mut result, next);
        }

        nodes.pop_front();
    }

    // drain(Q, result, nullptr);
    drain(&mut q, &mut result, core::ptr::null_mut());

    // std::swap(stats, result);
    core::mem::swap(stats, &mut result);

    // Free every allocated Node (the C++ owners go out of scope here).
    for node_ptr in all_nodes {
        unsafe {
            drop(alloc::boxed::Box::from_raw(node_ptr));
        }
    }
}
