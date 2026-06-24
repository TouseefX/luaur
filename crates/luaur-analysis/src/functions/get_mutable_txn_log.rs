//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TxnLog.h:51:get_mutable`
//! Source: `Analysis/include/Luau/TxnLog.h:51-56` (hand-ported)

use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::pending_type::PendingType;
use crate::records::r#type::Type;
use crate::type_aliases::type_variant::TypeVariantMember;

/// C++ `template<typename T> T* getMutable(PendingType* pending)`.
pub unsafe fn get_mutable_pending_type<T: TypeVariantMember + 'static>(
    pending: *mut PendingType,
) -> *mut T {
    // We use getMutable here because this state is intended to be mutated freely.
    get_mutable_type_id::<T>(&(*pending).pending as *const Type)
}

#[allow(unused_imports)]
pub use get_mutable_pending_type as get_mutable;
