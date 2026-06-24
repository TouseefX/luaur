use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::state_dot::StateDot;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::type_id::TypeId;

impl StateDot {
    pub fn can_duplicate_primitive(&self, ty: TypeId) -> bool {
        let bound = unsafe { get_type_id::<BoundType>(ty) };
        if !bound.is_null() {
            return false;
        }

        let primitive = unsafe { get_type_id::<PrimitiveType>(ty) };
        if !primitive.is_null() {
            return true;
        }

        let any = unsafe { get_type_id::<AnyType>(ty) };
        if !any.is_null() {
            return true;
        }

        let unknown = unsafe { get_type_id::<UnknownType>(ty) };
        if !unknown.is_null() {
            return true;
        }

        let never = unsafe { get_type_id::<NeverType>(ty) };
        !never.is_null()
    }
}
