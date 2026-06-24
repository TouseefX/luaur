use crate::functions::get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header;
use crate::records::shared_code_gen_context::SharedCodeGenContext;

impl SharedCodeGenContext {
    pub unsafe extern "C" fn on_destroy_function(execdata: *mut core::ffi::c_void) {
        let native_proto_exec_data_header =
            get_native_proto_exec_data_header(execdata as *const u32);
        unsafe {
            (*native_proto_exec_data_header)
                .native_module
                .as_ref()
                .unwrap()
                .release();
        }
    }
}
