use crate::records::base_code_gen_context::BaseCodeGenContext;
use crate::records::module_bind_result::ModuleBindResult;
use crate::records::shared_code_allocator::SharedCodeAllocator;
use crate::records::standalone_code_gen_context::StandaloneCodeGenContext;
use crate::type_aliases::allocation_callback::AllocationCallback;
use crate::type_aliases::module_id::ModuleId;
use crate::type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr;
use alloc::vec::Vec;
use core::ffi::c_void;
use luaur_vm::records::proto::Proto;

unsafe fn standalone_try_bind_existing_module_shim(
    ctx: *mut BaseCodeGenContext,
    module_id: &ModuleId,
    module_protos: &Vec<*mut Proto>,
) -> Option<ModuleBindResult> {
    (*(ctx as *mut StandaloneCodeGenContext)).try_bind_existing_module(module_id, module_protos)
}

unsafe fn standalone_bind_module_shim(
    ctx: *mut BaseCodeGenContext,
    module_id: &Option<ModuleId>,
    module_protos: &Vec<*mut Proto>,
    native_protos: Vec<NativeProtoExecDataPtr>,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
) -> ModuleBindResult {
    (*(ctx as *mut StandaloneCodeGenContext)).bind_module(
        module_id,
        module_protos,
        native_protos,
        data,
        data_size,
        code,
        code_size,
    )
}

impl StandaloneCodeGenContext {
    pub fn standalone_code_gen_context_standalone_code_gen_context(
        &mut self,
        block_size: usize,
        max_total_size: usize,
        allocation_callback: *mut AllocationCallback,
        allocation_callback_context: *mut c_void,
    ) {
        let mut base = BaseCodeGenContext::base_code_gen_context_base_code_gen_context(
            block_size,
            max_total_size,
            allocation_callback,
            allocation_callback_context,
        );
        base.try_bind_existing_module_fn = Some(standalone_try_bind_existing_module_shim);
        base.bind_module_fn = Some(standalone_bind_module_shim);

        let mut shared_allocator = SharedCodeAllocator::default();
        shared_allocator.shared_code_allocator_code_allocator(&mut base.code_allocator);

        unsafe {
            core::ptr::write(
                self,
                StandaloneCodeGenContext {
                    base,
                    shared_allocator,
                },
            );
        }
    }
}
