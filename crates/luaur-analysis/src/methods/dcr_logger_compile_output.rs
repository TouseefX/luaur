use crate::records::dcr_logger::DcrLogger;
use crate::records::json_emitter::JsonEmitter;
use crate::records::object_emitter::ObjectEmitter;
use alloc::string::String;

impl DcrLogger {
    pub fn compile_output(&mut self) -> String {
        let mut emitter = JsonEmitter {
            comma: false,
            chunks: alloc::vec::Vec::new(),
        };
        emitter.json_emitter_json_emitter();
        let mut o: ObjectEmitter = emitter.write_object();
        o.write_pair("generation", &self.generation_log);
        o.write_pair("solve", &self.solve_log);
        o.write_pair("check", &self.check_log);
        o.finish();
        emitter.str()
    }
}
