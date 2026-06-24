use crate::functions::destroy_shared_code_gen_context::destroy_shared_code_gen_context;
use crate::records::shared_code_gen_context::SharedCodeGenContext;
use crate::records::shared_code_gen_context_deleter::SharedCodeGenContextDeleter;

impl SharedCodeGenContextDeleter {
    pub unsafe extern "C" fn shared_code_gen_context_deleter_operator_call(
        &self,
        code_gen_context: *const SharedCodeGenContext,
    ) {
        destroy_shared_code_gen_context(code_gen_context);
    }
}
