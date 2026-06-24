use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;
use crate::functions::bind_native_protos::bind_native_protos;
use crate::records::module_bind_result::ModuleBindResult;
use crate::records::shared_code_gen_context::SharedCodeGenContext;
use crate::type_aliases::module_id::ModuleId;
use alloc::vec::Vec;
use luaur_vm::records::proto::Proto;

impl SharedCodeGenContext {
    pub fn try_bind_existing_module(
        &mut self,
        module_id: &ModuleId,
        module_protos: &Vec<*mut Proto>,
    ) -> Option<ModuleBindResult> {
        let native_module_ref = self.shared_allocator.try_get_native_module(module_id);

        if native_module_ref.native_module_ref_empty() {
            return None;
        }

        let native_module = unsafe { &*native_module_ref.native_module_ref_get() };
        let mut native_protos = native_module.native_module_get_native_protos().clone();
        let protos_bound = bind_native_protos(module_protos, &mut native_protos, false);
        native_module.native_module_add_refs(protos_bound as usize);

        Some(ModuleBindResult {
            compilation_result: CodeGenCompilationResult::Success,
            functions_bound: protos_bound,
        })
    }
}
