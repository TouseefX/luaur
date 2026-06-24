use crate::records::json_emitter::JsonEmitter;
use alloc::string::String;

impl JsonEmitter {
    pub fn str(&mut self) -> String {
        self.chunks.join("")
    }
}
