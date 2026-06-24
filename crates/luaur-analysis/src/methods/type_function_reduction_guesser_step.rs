use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::type_aliases::type_id::TypeId;

impl TypeFunctionReductionGuesser {
    pub fn step(&mut self) {
        let t: TypeId = unsafe { (*self.to_infer.front_mut()).clone() };
        unsafe { self.to_infer.pop_front() };
        let t: TypeId = unsafe { follow_type_id(t) };
        let tf: *const TypeFunctionInstanceType =
            unsafe { get_type_id::<TypeFunctionInstanceType>(t) };
        if !tf.is_null() {
            self.infer_type_function_substitutions(t, tf);
        }
    }
}
