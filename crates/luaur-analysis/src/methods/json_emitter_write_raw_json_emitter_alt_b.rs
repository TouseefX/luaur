//! Node: `cxx:Method:Luau.Analysis:Analysis/src/JsonEmitter.cpp:104:json_emitter_write_raw`
//! Source: `Analysis/src/JsonEmitter.cpp` (JsonEmitter.cpp:104-107, hand-ported)

use crate::records::json_emitter::JsonEmitter;

impl JsonEmitter {
    // writeRaw(char) — pinned overload name
    pub fn write_raw_c_char(&mut self, c: core::ffi::c_char) {
        let buf = [c as u8];
        // single char as a one-byte string view
        self.write_raw_string_view(unsafe { core::str::from_utf8_unchecked(&buf) });
    }
}
