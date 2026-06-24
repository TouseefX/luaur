//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:60:json_emitter_write_comma`
//! Source: `tests/JsonEmitter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/JsonEmitter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
//! - incoming:
//!   - declares <- source_file tests/JsonEmitter.test.cpp
//! - outgoing:
//!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
//!   - calls -> method JsonEmitter::writeComma (Analysis/src/JsonEmitter.cpp)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item json_emitter_write_comma

#[cfg(test)]
#[test]
fn json_emitter_write_comma() {
    use luaur_analysis::functions::write_json_emitter_alt_w::write_json_emitter_bool;
    use luaur_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    emitter.write_comma();
    write_json_emitter_bool(&mut emitter, true);
    emitter.write_comma();
    write_json_emitter_bool(&mut emitter, false);
    assert_eq!("true,false", emitter.str());
}
