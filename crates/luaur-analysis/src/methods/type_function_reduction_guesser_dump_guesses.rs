use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;

impl TypeFunctionReductionGuesser {
    pub fn dump_guesses(&mut self) {
        for (tf, t) in self.function_reduces_to.iter() {
            std::print!(
                "Type family {} ~~> {}\n",
                to_string_type_id(*tf),
                to_string_type_id(*t)
            );
        }
        for (t, t_) in self.substitutable.iter() {
            std::print!(
                "Substitute {} for {}\n",
                to_string_type_id(*t),
                to_string_type_id(*t_)
            );
        }
    }
}
