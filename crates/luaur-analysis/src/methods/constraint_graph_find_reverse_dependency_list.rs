use crate::records::constraint_graph::ConstraintGraph;
use crate::records::constraint_list::ConstraintList;
use crate::records::r#type::Type;
use crate::type_aliases::constraint_vertex::ConstraintVertex;
use core::ptr::NonNull;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl ConstraintGraph {
    pub fn find_reverse_dependency_list(
        &mut self,
        vertex: ConstraintVertex,
    ) -> NonNull<ConstraintList> {
        if let Some(rdep) = self.reverse_dependencies.find(&vertex) {
            return NonNull::new(*rdep).unwrap();
        }

        self.constraint_lists.push(Box::new(ConstraintList {
            present: DenseHashMap::new(ConstraintVertex::V0(core::ptr::null::<Type>())),
            order: Vec::new(),
            entries: 0,
        }));
        let newlist =
            NonNull::new(&mut **self.constraint_lists.last_mut().unwrap() as *mut ConstraintList)
                .unwrap();

        let (_it, fresh) = self
            .reverse_dependencies
            .try_insert(vertex, newlist.as_ptr());
        LUAU_ASSERT!(fresh);
        newlist
    }
}
