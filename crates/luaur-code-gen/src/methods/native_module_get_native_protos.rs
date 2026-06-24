use crate::records::native_module::NativeModule;
use crate::type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr;

impl NativeModule {
    pub fn native_module_get_native_protos(&self) -> &Vec<NativeProtoExecDataPtr> {
        &self.native_protos
    }
}
