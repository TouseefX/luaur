use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::intersection_builder::IntersectionBuilder;
use crate::records::intersection_type::IntersectionType;
use crate::records::never_type::NeverType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

impl IntersectionBuilder {
    pub fn add(&mut self, ty: TypeId) {
        let ty = unsafe { follow_type_id(ty) };

        if unsafe { !get_type_id::<NeverType>(ty).is_null() } {
            self.is_bottom = true;
            return;
        }

        if unsafe { !get_type_id::<UnknownType>(ty).is_null() } {
            return;
        }

        if let Some(itv) = unsafe { get_type_id::<IntersectionType>(ty).as_ref() } {
            for &part in itv.parts.iter() {
                self.parts.insert_type_id(part);
            }
        } else {
            self.parts.insert_type_id(ty);
        }
    }
}
