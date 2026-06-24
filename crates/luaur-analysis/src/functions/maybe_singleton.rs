use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::singleton_type::SingletonType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

pub fn maybe_singleton(ty: TypeId) -> bool {
    unsafe {
        let ty = follow_type_id(ty);
        if !get_type_id::<SingletonType>(ty).is_null() {
            return true;
        }
        let utv = get_type_id::<UnionType>(ty);
        if !utv.is_null() {
            for &option in &(*utv).options {
                if !get_type_id::<SingletonType>(follow_type_id(option)).is_null() {
                    return true;
                }
            }
        }
        let itv = get_type_id::<IntersectionType>(ty);
        if !itv.is_null() {
            for &part in &(*itv).parts {
                if maybe_singleton(part) {
                    // will i regret this?
                    return true;
                }
            }
        }
        let tfit = get_type_id::<TypeFunctionInstanceType>(ty);
        if !tfit.is_null() {
            let name = &(*(*tfit).function.as_ptr()).name;
            if name == "keyof" || name == "rawkeyof" {
                return true;
            }
        }
        false
    }
}
