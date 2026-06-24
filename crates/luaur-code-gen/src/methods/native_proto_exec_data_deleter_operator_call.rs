use crate::functions::destroy_native_proto_exec_data::destroy_native_proto_exec_data;
use crate::records::native_proto_exec_data_deleter::NativeProtoExecDataDeleter;

impl NativeProtoExecDataDeleter {
    pub fn operator_call(&self, instruction_offsets: *const u32) {
        unsafe {
            destroy_native_proto_exec_data(instruction_offsets);
        }
    }
}
