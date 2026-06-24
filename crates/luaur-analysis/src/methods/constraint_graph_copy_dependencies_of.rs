use crate::records::constraint_graph::ConstraintGraph;
use crate::records::reference_count_initializer::ReferenceCountInitializer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::constraint_vertex::ConstraintVertex;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_ids::TypePackIds;

impl ConstraintGraph {
    pub fn copy_dependencies_of_type_id(&mut self, source: TypeId, target: TypeId) {
        let source_dependencies = self.find_dependency_list(ConstraintVertex::V0(source));
        let mut mutated_types = TypeIds::type_ids();
        let mut mutated_type_packs = TypePackIds::new(core::ptr::null_mut());

        let _rci =
            ReferenceCountInitializer::reference_count_initializer_reference_count_initializer(
                &mut mutated_types as *mut TypeIds,
                &mut mutated_type_packs as *mut TypePackIds,
            );

        self.copy_dependencies_to_reachable_types(
            None,
            source_dependencies,
            mutated_types,
            mutated_type_packs,
        );

        let _ = target;
    }

    pub fn copy_dependencies_of_type_pack_id(&mut self, source: TypePackId, target: TypePackId) {
        let source_dependencies = self.find_dependency_list(ConstraintVertex::V1(source));
        let mut mutated_types = TypeIds::type_ids();
        let mut mutated_type_packs = TypePackIds::new(core::ptr::null_mut());

        let _rci =
            ReferenceCountInitializer::reference_count_initializer_reference_count_initializer(
                &mut mutated_types as *mut TypeIds,
                &mut mutated_type_packs as *mut TypePackIds,
            );

        self.copy_dependencies_to_reachable_types(
            None,
            source_dependencies,
            mutated_types,
            mutated_type_packs,
        );

        let _ = target;
    }
}
