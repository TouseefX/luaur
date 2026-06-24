use crate::records::stringifier_state::StringifierState;

impl StringifierState {
    pub fn emit_usize(&mut self, i: usize) {
        self.emit_string(&i.to_string());
    }
}
