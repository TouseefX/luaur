use crate::records::frontend::Frontend;
use crate::records::source_node::SourceNode;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use alloc::vec::Vec;
use luaur_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

impl Frontend {
    pub fn traverse_dependents(
        &mut self,
        name: &ModuleName,
        process_subtree: Box<dyn Fn(&mut SourceNode) -> bool>,
    ) {
        LUAU_TIMETRACE_SCOPE!("Frontend::traverseDependents", "Frontend");

        if !self.source_nodes.contains_key(name) {
            return;
        }

        let mut queue = Vec::new();
        queue.push(name.clone());

        while !queue.is_empty() {
            let next = queue.pop().unwrap();

            debug_assert!(self.source_nodes.contains_key(&next));
            let source_node_arc = match self.source_nodes.get(&next) {
                Some(v) => v,
                None => continue,
            };

            // Clone to avoid borrowing `self.source_nodes` across callback and subsequent queue mutations.
            let mut source_node_ref: *mut SourceNode =
                source_node_arc.as_ref() as *const SourceNode as *mut SourceNode;

            // SAFETY: `SourceNode` is owned by `Frontend.source_nodes` as an `Arc`. We do not free it here,
            // and callback is expected to mutate the node. We avoid aliasing `&mut` borrows by using the raw pointer.
            let keep_going = unsafe { process_subtree(&mut *source_node_ref) };

            if !keep_going {
                continue;
            }

            let dependents = unsafe { &(*source_node_ref).dependents };
            for d in dependents.iter() {
                queue.push(d.clone());
            }
        }
    }
}
