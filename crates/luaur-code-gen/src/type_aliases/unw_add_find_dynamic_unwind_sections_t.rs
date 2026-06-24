use crate::records::unw_dynamic_unwind_sections_t::unw_dynamic_unwind_sections_t;

#[allow(non_camel_case_types)]
pub type unw_add_find_dynamic_unwind_sections_t = Option<
    unsafe extern "C" fn(
        find_callback: Option<
            unsafe extern "C" fn(
                addr: usize,
                info: *mut unw_dynamic_unwind_sections_t,
            ) -> core::ffi::c_int,
        >,
    ) -> core::ffi::c_int,
>;

pub type UnwAddFindDynamicUnwindSectionsT = unw_add_find_dynamic_unwind_sections_t;
