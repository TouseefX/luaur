use crate::records::special::Special;
use luaur_analysis::methods::object_emitter_write_pair::WriteJson;
use luaur_analysis::records::json_emitter::JsonEmitter;

pub fn write(emitter: &mut JsonEmitter, value: &Special) {
    let mut o = emitter.write_object();
    o.write_pair("foo", value.foo);
    o.write_pair("bar", value.bar);
}

impl WriteJson for Special {
    fn write_json(&self, emitter: &mut JsonEmitter) {
        write(emitter, self);
    }
}
