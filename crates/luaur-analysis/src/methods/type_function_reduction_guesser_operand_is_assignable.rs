use crate::functions::get_type_alt_j::get_type_id;
use crate::records::generic_type::GenericType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::type_aliases::type_id::TypeId;

impl TypeFunctionReductionGuesser {
    pub fn operand_is_assignable(&self, ty: TypeId) -> bool {
        if !unsafe { get_type_id::<TypeFunctionInstanceType>(ty) }.is_null() {
            return true;
        }
        if !unsafe { get_type_id::<GenericType>(ty) }.is_null() {
            return true;
        }
        if self.cyclic_instances.contains(&ty) {
            return true;
        }
        false
    }
}
