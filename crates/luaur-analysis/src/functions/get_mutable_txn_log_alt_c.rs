//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TxnLog.h:58:get_mutable`
//! Source: `Analysis/include/Luau/TxnLog.h:58-63` (hand-ported)

use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::records::pending_type_pack::PendingTypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_variant::TypePackVariantMember;

/// C++ `template<typename T> T* getMutable(PendingTypePack* pending)`.
pub unsafe fn get_mutable_pending_type_pack<T: TypePackVariantMember + 'static>(
    pending: *mut PendingTypePack,
) -> *mut T {
    // We use getMutable here because this state is intended to be mutated freely.
    get_mutable_type_pack_id::<T>(&(*pending).pending as *const TypePackVar)
}

#[allow(unused_imports)]
pub use get_mutable_pending_type_pack as get_mutable;
