//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Type.cpp:271:is_overloaded_function`
//! Source: `Analysis/src/Type.cpp:271-283` (hand-ported)

use crate::functions::flatten_intersection::flatten_intersection;
use crate::functions::follow_type::follow;
use crate::functions::get_type_alt_j::get;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::type_aliases::type_id::TypeId;

pub fn is_overloaded_function(ty: TypeId) -> bool {
    unsafe {
        if get::<IntersectionType>(follow(ty)).is_null() {
            return false;
        }

        let parts = flatten_intersection(ty);
        parts
            .iter()
            .all(|&part| !get::<FunctionType>(part).is_null())
    }
}
