use crate::records::info_code_allocator_test::Info;
use core::ffi::c_void;
use core::ptr;

pub unsafe extern "C" fn create_block_unwind_info_code_allocator_test(
    context: *mut c_void,
    block: *mut u8,
    _block_size: usize,
    begin_offset: &mut usize,
) -> *mut c_void {
    let info = &mut *(context.cast::<Info>());

    assert_eq!(info.unwind.len(), 8);
    ptr::copy_nonoverlapping(info.unwind.as_ptr(), block, info.unwind.len());
    *begin_offset = 8;
    info.block = block;

    Box::into_raw(Box::new(7_i32)).cast()
}
