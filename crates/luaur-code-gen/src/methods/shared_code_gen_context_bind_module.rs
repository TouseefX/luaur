use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;
use crate::functions::bind_native_protos::bind_native_protos;
use crate::records::module_bind_result::ModuleBindResult;
use crate::records::native_module_ref::NativeModuleRef;
use crate::records::shared_code_gen_context::SharedCodeGenContext;
use crate::type_aliases::module_id::ModuleId;
use crate::type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr;
use alloc::vec::Vec;
use luaur_vm::records::proto::Proto;

impl SharedCodeGenContext {
    pub fn bind_module(
        &mut self,
        module_id: &Option<ModuleId>,
        module_protos: &Vec<*mut Proto>,
        native_protos: Vec<NativeProtoExecDataPtr>,
        data: *const u8,
        data_size: usize,
        code: *const u8,
        code_size: usize,
    ) -> ModuleBindResult {
        let (native_module_ref, _inserted): (NativeModuleRef, bool) =
            if let Some(module_id) = module_id {
                self.shared_allocator.get_or_insert_native_module(
                    module_id,
                    native_protos,
                    data,
                    data_size,
                    code,
                    code_size,
                )
            } else {
                (
                    self.shared_allocator.insert_anonymous_native_module(
                        native_protos,
                        data,
                        data_size,
                        code,
                        code_size,
                    ),
                    true,
                )
            };

        if native_module_ref.native_module_ref_empty() {
            return ModuleBindResult {
                compilation_result: CodeGenCompilationResult::AllocationFailed,
                functions_bound: 0,
            };
        }

        let native_module = unsafe { &*native_module_ref.native_module_ref_get() };
        let mut native_protos = native_module.native_module_get_native_protos().clone();
        let protos_bound = bind_native_protos(module_protos, &mut native_protos, false);
        native_module.native_module_add_refs(protos_bound as usize);

        ModuleBindResult {
            compilation_result: CodeGenCompilationResult::Success,
            functions_bound: protos_bound,
        }
    }
}
