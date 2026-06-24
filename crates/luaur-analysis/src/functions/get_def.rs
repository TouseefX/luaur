//! Source: `Analysis/include/Luau/Def.h:73-77` (hand-ported)
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::variant::VariantMember;

/// C++ `template<typename T> const T* get(DefId def)`.
pub fn get_def_id<T: VariantMember>(def: DefId) -> *const T {
    unsafe {
        match T::get_if(&(*def).v) {
            Some(r) => r as *const T,
            None => core::ptr::null(),
        }
    }
}
