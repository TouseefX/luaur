use crate::records::info_code_allocator_test_alt_b::Info;
use core::ffi::c_void;

pub unsafe extern "C" fn destroy_block_unwind_info_code_allocator_test_alt_b(
    context: *mut c_void,
    unwind_data: *mut c_void,
) {
    let info = &mut *(context.cast::<Info>());
    info.destroy_called = true;

    let value = Box::from_raw(unwind_data.cast::<i32>());
    assert_eq!(*value, 7);
}
