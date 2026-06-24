use crate::records::native_proto_exec_data_header::NativeProtoExecDataHeader;

#[inline]
pub fn get_native_proto_exec_data_header_mut(
    instruction_offsets: *mut u32,
) -> *mut NativeProtoExecDataHeader {
    let header_size = core::mem::size_of::<NativeProtoExecDataHeader>();
    let header_ptr = unsafe { (instruction_offsets as *mut u8).offset(-(header_size as isize)) }
        as *mut NativeProtoExecDataHeader;
    header_ptr
}
