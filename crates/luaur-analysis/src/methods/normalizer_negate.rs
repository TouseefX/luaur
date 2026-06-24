use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::intersection_type::IntersectionType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::normalizer::Normalizer;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn negate(&mut self, mut there: TypeId) -> TypeId {
        self.consume_fuel();

        there = unsafe { follow_type_id(there) };

        if unsafe { !get_type_id::<AnyType>(there).is_null() } {
            return there;
        } else if unsafe { !get_type_id::<UnknownType>(there).is_null() } {
            return unsafe { (*self.builtin_types).neverType };
        } else if unsafe { !get_type_id::<NeverType>(there).is_null() } {
            return unsafe { (*self.builtin_types).unknownType };
        } else if let Some(ntv) = unsafe { get_type_id::<NegationType>(there).as_ref() } {
            return ntv.ty;
        } else if let Some(utv) = unsafe { get_type_id::<UnionType>(there).as_ref() } {
            let mut parts = Vec::new();
            for option in utv.options.iter() {
                parts.push(self.negate(*option));
            }
            return unsafe { (*self.arena).add_type(IntersectionType { parts }) };
        } else if let Some(itv) = unsafe { get_type_id::<IntersectionType>(there).as_ref() } {
            let mut options = Vec::new();
            for part in itv.parts.iter() {
                options.push(self.negate(*part));
            }
            return unsafe { (*self.arena).add_type(UnionType { options }) };
        } else {
            return there;
        }
    }
}
