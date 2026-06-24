use crate::functions::are_equal_structural_type_equality::are_equal_seen_set_type_pack_var_type_pack_var;
use crate::functions::are_seen::are_seen;
use crate::records::function_type::FunctionType;
use crate::type_aliases::seen_set_structural_type_equality::SeenSet;

#[allow(non_snake_case)]
pub fn are_equal_seen_set_function_type_function_type(
    seen: &mut SeenSet,
    lhs: &FunctionType,
    rhs: &FunctionType,
) -> bool {
    if are_seen(
        unsafe {
            core::mem::transmute::<
                &mut SeenSet,
                &mut std::collections::BTreeSet<(*mut core::ffi::c_void, *mut core::ffi::c_void)>,
            >(seen)
        },
        lhs as *const FunctionType as *const core::ffi::c_void,
        rhs as *const FunctionType as *const core::ffi::c_void,
    ) {
        return true;
    }

    let lhs_arg_types = unsafe { &*lhs.arg_types };
    let rhs_arg_types = unsafe { &*rhs.arg_types };
    if !are_equal_seen_set_type_pack_var_type_pack_var(seen, lhs_arg_types, rhs_arg_types) {
        return false;
    }

    let lhs_ret_types = unsafe { &*lhs.ret_types };
    let rhs_ret_types = unsafe { &*rhs.ret_types };
    if !are_equal_seen_set_type_pack_var_type_pack_var(seen, lhs_ret_types, rhs_ret_types) {
        return false;
    }

    true
}
