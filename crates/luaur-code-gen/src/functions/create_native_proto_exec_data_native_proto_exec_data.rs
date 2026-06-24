use crate::functions::compute_native_exec_data_size::compute_native_exec_data_size;
use crate::records::native_proto_exec_data_header::NativeProtoExecDataHeader;
use crate::type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr;
use core::ptr::NonNull;

pub fn create_native_proto_exec_data_u32_u32(
    bytecode_instruction_count: u32,
    extra_data_count: u32,
) -> NativeProtoExecDataPtr {
    let total_size = compute_native_exec_data_size(bytecode_instruction_count, extra_data_count);
    let layout = core::alloc::Layout::from_size_align(total_size, core::mem::align_of::<u32>())
        .expect("Invalid layout for NativeProtoExecData");

    let bytes = unsafe { alloc::alloc::alloc(layout) };
    if bytes.is_null() {
        alloc::alloc::handle_alloc_error(layout);
    }

    let header = bytes as *mut NativeProtoExecDataHeader;
    unsafe {
        header.write(NativeProtoExecDataHeader {
            native_module: core::ptr::null_mut(),
            entry_offset_or_address: core::ptr::null(),
            bytecode_id: 0,
            bytecode_instruction_count,
            extra_data_count,
            native_code_size: 0,
        });
    }

    let data_ptr =
        unsafe { bytes.add(core::mem::size_of::<NativeProtoExecDataHeader>()) as *mut u32 };
    unsafe { NonNull::new_unchecked(data_ptr) }
}
