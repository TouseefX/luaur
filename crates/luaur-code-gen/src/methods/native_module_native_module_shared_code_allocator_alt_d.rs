use crate::records::code_allocation_data::CodeAllocationData;
use crate::records::native_module::NativeModule;
use crate::records::shared_code_allocator::SharedCodeAllocator;
use crate::type_aliases::module_id::ModuleId;
use crate::type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr;
use alloc::vec::Vec;

pub fn native_module_native_module(
    allocator: *mut SharedCodeAllocator,
    module_id: &Option<ModuleId>,
    code_allocation_data: CodeAllocationData,
    native_protos: Vec<NativeProtoExecDataPtr>,
) -> NativeModule {
    NativeModule::native_module_shared_code_allocator_optional_module_id_code_allocation_data_vector_native_proto_exec_data_ptr(
        allocator,
        module_id,
        code_allocation_data,
        native_protos,
    )
}
