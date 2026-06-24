use crate::records::mapped_generic_environment::MappedGenericEnvironment;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl MappedGenericEnvironment {
    pub fn mapped_generic_frame_mapped_generic_frame(
        &mut self,
        mappings: DenseHashMap<TypePackId, Option<TypePackId>>,
        parent_scope_index: Option<usize>,
    ) {
        let frame = crate::records::mapped_generic_frame::MappedGenericFrame {
            mappings,
            parent_scope_index,
            children: DenseHashSet::new(0),
        };
        self.frames.push(frame);
        self.current_scope_index = Some(self.frames.len() - 1);
    }
}
