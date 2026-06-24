use crate::functions::get_type_alt_j::get_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::refine_type_scrubber::RefineTypeScrubber;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl RefineTypeScrubber {
    pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
        if let Some(ut) = unsafe { get_type_id::<UnionType>(ty).as_ref() } {
            for &option in &ut.options {
                if option == self.needle {
                    return true;
                }
            }
        } else if let Some(it) = unsafe { get_type_id::<IntersectionType>(ty).as_ref() } {
            for &part in &it.parts {
                if part == self.needle {
                    return true;
                }
            }
        }
        ty == self.needle
    }
}
