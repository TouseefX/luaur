//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeUtils.h:217:get_2`
//! Source: `Analysis/include/Luau/TypeUtils.h:217-228` (hand-ported)

use crate::functions::get_type_utils::get_optional_ty;
use crate::records::try_pair::TryPair;

/// C++ `template<typename A, typename B, typename Ty> TryPair<const A*, const B*> get2(Ty one, Ty two)`.
pub fn get2<A, B, Ty>(one: Ty, two: Ty) -> TryPair<*const A, *const B>
where
    A: crate::functions::get_type_utils::GetThroughId<Ty>,
    B: crate::functions::get_type_utils::GetThroughId<Ty>,
    Ty: Copy,
{
    let a = unsafe { get_optional_ty::<A, Ty>(Some(one)) };
    let b = unsafe { get_optional_ty::<B, Ty>(Some(two)) };

    if !a.is_null() && !b.is_null() {
        TryPair {
            first: a,
            second: b,
        }
    } else {
        TryPair {
            first: core::ptr::null(),
            second: core::ptr::null(),
        }
    }
}
