//! Node: `cxx:Function:Luau.Analysis:Analysis/src/JsonEmitter.cpp:173:write`
//! Source: `Analysis/src/JsonEmitter.cpp` (JsonEmitter.cpp:173-176)
//!
//! C++:
//! ```cpp
//! void write(JsonEmitter& emitter, char c)
//! {
//!     write(emitter, std::string_view{&c, 1});
//! }
//! ```
//! Writes a single character to the emitter as a one-character (escaped) string.

use crate::functions::write_json_emitter_alt_ae::write_json_emitter_string_view;
use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_char(emitter: &mut JsonEmitter, c: core::ffi::c_char) {
    // std::string_view{&c, 1} — a one-byte view over the single char.
    let buf = [c as u8];
    let sv = unsafe { core::str::from_utf8_unchecked(&buf) };
    write_json_emitter_string_view(emitter, sv);
}
