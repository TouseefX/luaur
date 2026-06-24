use crate::records::stringifier_state::StringifierState;

impl StringifierState {
    pub fn emit_i32(&mut self, i: i32) {
        let s = i.to_string();
        self.emit_string(&s);
    }
}
