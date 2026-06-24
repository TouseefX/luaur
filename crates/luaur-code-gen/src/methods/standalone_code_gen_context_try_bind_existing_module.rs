use crate::records::module_bind_result::ModuleBindResult;
use crate::records::standalone_code_gen_context::StandaloneCodeGenContext;
use crate::type_aliases::module_id::ModuleId;
use alloc::vec::Vec;
use luaur_vm::records::proto::Proto;

impl StandaloneCodeGenContext {
    pub fn try_bind_existing_module(
        &mut self,
        _module_id: &ModuleId,
        _module_protos: &Vec<*mut Proto>,
    ) -> Option<ModuleBindResult> {
        // The StandaloneCodeGenContext does not support sharing of native code
        None
    }
}
