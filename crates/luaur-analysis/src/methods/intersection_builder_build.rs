use crate::records::intersection_builder::IntersectionBuilder;
use crate::records::intersection_type::IntersectionType;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl IntersectionBuilder {
    pub fn build(&mut self) -> TypeId {
        if self.is_bottom {
            return unsafe { (*self.builtin_types).neverType };
        }

        if self.parts.size() == 0 {
            return unsafe { (*self.builtin_types).unknownType };
        }

        if self.parts.size() == 1 {
            return self.parts.front();
        }

        unsafe {
            (*self.arena).add_type(IntersectionType {
                parts: self.parts.take(),
            })
        }
    }
}
