use crate::records::json_emitter::JsonEmitter;

impl JsonEmitter {
    pub fn json_emitter_json_emitter(&mut self) {
        self.comma = false;
        self.chunks.clear();
        self.new_chunk();
    }
}
