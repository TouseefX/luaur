extern crate alloc;

use crate::records::code_allocation_data::CodeAllocationData;
use crate::records::code_allocator::CodeAllocator;
use crate::records::module_bind_result::ModuleBindResult;
use crate::records::native_context::NativeContext;
use crate::records::unwind_builder::UnwindBuilder;
use crate::type_aliases::allocation_callback::AllocationCallback;
use crate::type_aliases::module_id::ModuleId;
use crate::type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr;
use crate::type_aliases::userdata_remapper_callback::UserdataRemapperCallback;
use alloc::vec::Vec;
use core::ffi::c_void;
use luaur_vm::records::proto::Proto;

#[derive(Debug)]
#[repr(C)]
pub struct BaseCodeGenContext {
    pub(crate) code_allocator: CodeAllocator,
    pub try_bind_existing_module_fn: Option<
        unsafe fn(*mut BaseCodeGenContext, &ModuleId, &Vec<*mut Proto>) -> Option<ModuleBindResult>,
    >,
    pub bind_module_fn: Option<
        unsafe fn(
            *mut BaseCodeGenContext,
            &Option<ModuleId>,
            &Vec<*mut Proto>,
            Vec<NativeProtoExecDataPtr>,
            *const u8,
            usize,
            *const u8,
            usize,
        ) -> ModuleBindResult,
    >,
    pub(crate) unwind_builder: *mut UnwindBuilder,
    pub gate_data_deprecated: *mut u8,
    pub gate_data_size_deprecated: usize,
    pub gate_allocation_data: CodeAllocationData,
    pub userdata_remapping_context: *mut c_void,
    // C++ field: `UserdataRemapperCallback* userdataRemapper` where the C++ alias
    // is a *function type*, so the field is a nullable function pointer. Rust's
    // `UserdataRemapperCallback` alias is already the function-pointer type, so
    // the faithful equivalent is `Option<UserdataRemapperCallback>` (Some == the
    // function pointer, None == nullptr).
    pub userdata_remapper: Option<UserdataRemapperCallback>,
    pub context: NativeContext,
}
