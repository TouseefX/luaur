use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::mapped_generic_environment::MappedGenericEnvironment;
use crate::records::not_bindable::NotBindable;
use crate::records::unmapped::Unmapped;
use crate::type_aliases::lookup_result::LookupResult;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_common::records::variant::Variant3;

impl MappedGenericEnvironment {
    pub fn lookup_generic_pack(&self, generic_tp: TypePackId) -> LookupResult {
        let generic_tp = unsafe { follow_type_pack_id(generic_tp) };

        let mut current_frame_index = self.current_scope_index;

        while let Some(index) = current_frame_index {
            let current_frame = &self.frames[index];
            if let Some(mapped_pack) =
                crate::methods::subtyping_bind_generic::dense_hash_map_find_no_default(
                    &current_frame.mappings,
                    &generic_tp,
                )
            {
                if let Some(tp) = *mapped_pack {
                    return Variant3::V0(tp);
                } else {
                    return Variant3::V1(Unmapped { scope_index: index });
                }
            }
            current_frame_index = current_frame.parent_scope_index;
        }

        if let Some(base_index) = self.current_scope_index {
            let base_frame = &self.frames[base_index];
            let mut to_check: Vec<usize> = base_frame.children.iter().copied().collect();

            while let Some(curr_index) = to_check.pop() {
                let current_frame = &self.frames[curr_index];
                if let Some(mapped_pack) =
                    crate::methods::subtyping_bind_generic::dense_hash_map_find_no_default(
                        &current_frame.mappings,
                        &generic_tp,
                    )
                {
                    if let Some(tp) = *mapped_pack {
                        return Variant3::V0(tp);
                    } else {
                        return Variant3::V1(Unmapped {
                            scope_index: curr_index,
                        });
                    }
                }
                to_check.extend(current_frame.children.iter().copied());
            }
        }

        Variant3::V2(NotBindable { _unused: None })
    }
}
