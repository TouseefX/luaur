use crate::functions::init_header_functions_code_gen_a_64::init_header_functions as init_header_functions_a64;
use crate::functions::init_header_functions_code_gen_x_64::init_header_functions as init_header_functions_x64;
use crate::macros::codegen_target_a_64::CODEGEN_TARGET_A64;
use crate::macros::codegen_target_x_64::CODEGEN_TARGET_X64;

impl crate::records::base_code_gen_context::BaseCodeGenContext {
    pub fn init_header_functions(&mut self) -> bool {
        if CODEGEN_TARGET_X64 && !init_header_functions_x64(self) {
            return false;
        }

        if CODEGEN_TARGET_A64 && !init_header_functions_a64(self) {
            return false;
        }

        true
    }
}
