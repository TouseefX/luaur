use crate::records::magic_instance_is_a::MagicInstanceIsA;
use luaur_analysis::records::magic_function_call_context::MagicFunctionCallContext;

impl MagicInstanceIsA {
    pub fn infer(&self, _context: &MagicFunctionCallContext) -> bool {
        false
    }
}
