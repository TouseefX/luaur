use crate::records::magic_function::MagicFunction;
use crate::records::magic_function_type_check_context::MagicFunctionTypeCheckContext;

impl MagicFunction {
    pub fn type_check(&self, _context: &MagicFunctionTypeCheckContext) -> bool {
        false
    }
}
