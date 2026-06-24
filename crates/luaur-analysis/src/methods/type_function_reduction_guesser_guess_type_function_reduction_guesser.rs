use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::type_aliases::type_id::TypeId;

impl TypeFunctionReductionGuesser {
    pub fn guess_type_id(&mut self, typ: TypeId) -> Option<TypeId> {
        let guessed_type: Option<TypeId> = self.guess_type(typ);

        if guessed_type.is_none() {
            return None;
        }

        let guess: TypeId = unsafe { follow_type_id(guessed_type.unwrap()) };

        let instance_ptr: *const TypeFunctionInstanceType =
            unsafe { get_type_id::<TypeFunctionInstanceType>(guess) };
        if instance_ptr.is_null() {
            Some(guess)
        } else {
            None
        }
    }
}
