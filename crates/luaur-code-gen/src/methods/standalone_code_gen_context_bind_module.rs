use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;
use crate::functions::bind_native_protos::bind_native_protos;
use crate::functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut;
use crate::records::module_bind_result::ModuleBindResult;
use crate::records::standalone_code_gen_context::StandaloneCodeGenContext;
use crate::type_aliases::module_id::ModuleId;
use crate::type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr;
use alloc::vec::Vec;
use luaur_common::FFlag;
use luaur_vm::records::proto::Proto;

impl StandaloneCodeGenContext {
    pub fn bind_module(
        &mut self,
        _module_id: &Option<ModuleId>,
        module_protos: &Vec<*mut Proto>,
        mut native_protos: Vec<NativeProtoExecDataPtr>,
        data: *const u8,
        data_size: usize,
        code: *const u8,
        code_size: usize,
    ) -> ModuleBindResult {
        if FFlag::LuauCodegenFreeBlocks.get() {
            let module_ref = self.shared_allocator.insert_anonymous_native_module(
                native_protos,
                data,
                data_size,
                code,
                code_size,
            );

            if module_ref.native_module_ref_empty() {
                return ModuleBindResult {
                    compilation_result: CodeGenCompilationResult::AllocationFailed,
                    functions_bound: 0,
                };
            }

            let native_module = unsafe { &*module_ref.native_module_ref_get() };
            let mut native_protos_for_bind =
                native_module.native_module_get_native_protos().clone();
            let protos_bound =
                bind_native_protos(module_protos, &mut native_protos_for_bind, false);
            native_module.native_module_add_refs(protos_bound as usize);

            ModuleBindResult {
                compilation_result: CodeGenCompilationResult::Success,
                functions_bound: protos_bound,
            }
        } else {
            let mut native_data = core::ptr::null_mut();
            let mut native_data_size = 0usize;
            let mut code_start = core::ptr::null_mut();

            if !self.base.code_allocator.allocate_deprecated(
                data,
                data_size,
                code,
                code_size,
                &mut native_data,
                &mut native_data_size,
                &mut code_start,
            ) {
                return ModuleBindResult {
                    compilation_result: CodeGenCompilationResult::AllocationFailed,
                    functions_bound: 0,
                };
            }

            for native_proto in &native_protos {
                unsafe {
                    let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
                    (*header).entry_offset_or_address =
                        code_start.add((*header).entry_offset_or_address as usize);
                }
            }

            let protos_bound = bind_native_protos(module_protos, &mut native_protos, true);

            ModuleBindResult {
                compilation_result: CodeGenCompilationResult::Success,
                functions_bound: protos_bound,
            }
        }
    }
}
