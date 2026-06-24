use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::union_builder::UnionBuilder;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

impl UnionBuilder {
    pub fn add(&mut self, ty: TypeId) {
        let ty = unsafe { follow_type_id(ty) };

        if unsafe { !get_type_id::<NeverType>(ty).is_null() } || self.is_top {
            return;
        }

        if unsafe { !get_type_id::<UnknownType>(ty).is_null() } {
            self.is_top = true;
            return;
        }

        if let Some(utv) = unsafe { get_type_id::<UnionType>(ty).as_ref() } {
            for &option in utv.options.iter() {
                self.options.insert_type_id(option);
            }
        } else {
            self.options.insert_type_id(ty);
        }
    }
}
