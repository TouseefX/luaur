use crate::functions::are_equal_structural_type_equality_alt_e::are_equal_seen_set_type_item_type_item;
use crate::functions::are_seen::are_seen;
use crate::records::metatable_type::MetatableType;
use crate::type_aliases::seen_set_structural_type_equality::SeenSet;

#[allow(non_snake_case)]
pub fn are_equal_seen_set_metatable_type_metatable_type(
    seen: &mut SeenSet,
    lhs: &MetatableType,
    rhs: &MetatableType,
) -> bool {
    // The SeenSet in this context is BTreeSet<(*const c_void, *const c_void)>.
    // are_seen expects BTreeSet<(*mut c_void, *mut c_void)>.
    // We must cast the seen set pointer to match the expected mutability of the are_seen signature.
    if are_seen(
        unsafe {
            core::mem::transmute::<
                &mut SeenSet,
                &mut std::collections::BTreeSet<(*mut core::ffi::c_void, *mut core::ffi::c_void)>,
            >(seen)
        },
        lhs as *const MetatableType as *mut core::ffi::c_void,
        rhs as *const MetatableType as *mut core::ffi::c_void,
    ) {
        return true;
    }

    // lhs.table and lhs.metatable are TypeId, which is *const Type.
    // are_equal_seen_set_type_item_type_item expects &Type.
    unsafe {
        are_equal_seen_set_type_item_type_item(seen, &*lhs.table, &*rhs.table)
            && are_equal_seen_set_type_item_type_item(seen, &*lhs.metatable, &*rhs.metatable)
    }
}
