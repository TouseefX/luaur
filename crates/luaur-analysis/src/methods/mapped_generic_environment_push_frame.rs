use crate::records::mapped_generic_environment::MappedGenericEnvironment;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl MappedGenericEnvironment {
    pub fn push_frame(&mut self, generic_tps: &Vec<TypePackId>) {
        let mut mappings: DenseHashMap<TypePackId, Option<TypePackId>> =
            DenseHashMap::new(core::ptr::null());
        for &tp in generic_tps.iter() {
            *mappings.get_or_insert(tp) = None;
        }
        let parent_scope_index = self.current_scope_index;
        let frame = crate::records::mapped_generic_frame::MappedGenericFrame {
            mappings,
            parent_scope_index,
            children: DenseHashSet::new(0),
        };
        self.frames.push(frame);
        let new_frame_index = self.frames.len() - 1;
        if let Some(current_scope_index) = self.current_scope_index {
            self.frames[current_scope_index]
                .children
                .insert(new_frame_index);
        }
        self.current_scope_index = Some(new_frame_index);
    }
}
