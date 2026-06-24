use crate::records::json_emitter::JsonEmitter;

impl JsonEmitter {
    pub fn write_comma(&mut self) {
        if self.comma {
            self.write_raw_c_char(b',' as core::ffi::c_char);
        } else {
            self.comma = true;
        }
    }
}
