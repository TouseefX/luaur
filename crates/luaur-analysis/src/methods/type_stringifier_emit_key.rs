use crate::records::type_stringifier::TypeStringifier;
use luaur_common::functions::escape::escape;

impl TypeStringifier {
    pub fn emit_key(&mut self, name: &str) {
        let state = unsafe { &mut *self.state };
        if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            state.emit(name);
        } else {
            state.emit("[\"");
            state.emit(&escape(name, false));
            state.emit("\"]");
        }
        state.emit(": ");
    }
}
