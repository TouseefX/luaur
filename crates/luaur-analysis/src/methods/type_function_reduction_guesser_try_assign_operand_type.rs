use crate::records::generic_type::GenericType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::type_aliases::type_id::TypeId;

use crate::functions::get_type_alt_j::get_type_id as get;

impl TypeFunctionReductionGuesser {
    pub fn try_assign_operand_type(&mut self, ty: TypeId) -> Option<TypeId> {
        // Because we collect innermost instances first, if we see a type function instance as an operand,
        // We try to check if we guessed a type for it
        if !unsafe { get::<TypeFunctionInstanceType>(ty) }.is_null() {
            if let Some(value) = self.function_reduces_to.find(&ty) {
                return Some(*value);
            }
        }

        // If ty is a generic, we need to check if we inferred a substitution
        if !unsafe { get::<GenericType>(ty) }.is_null() {
            if let Some(value) = self.substitutable.find(&ty) {
                return Some(*value);
            }
        }

        // If we cannot substitute a type for this value, we return an empty optional
        None
    }
}
