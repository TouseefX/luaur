use crate::records::stringifier_state::StringifierState;
use crate::records::type_stringifier::TypeStringifier;

impl TypeStringifier {
    pub fn type_stringifier(&mut self, state: &mut StringifierState) {
        self.state = state as *mut _;
    }
}
