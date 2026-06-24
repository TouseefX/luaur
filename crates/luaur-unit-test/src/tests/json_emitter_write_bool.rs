//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:34:json_emitter_write_bool`
//! Source: `tests/JsonEmitter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/JsonEmitter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
//! - incoming:
//!   - declares <- source_file tests/JsonEmitter.test.cpp
//! - outgoing:
//!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item json_emitter_write_bool

#[cfg(test)]
#[test]
fn json_emitter_write_bool() {
    use luaur_analysis::functions::write_json_emitter_alt_w::write_json_emitter_bool;
    use luaur_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    write_json_emitter_bool(&mut emitter, false);
    assert_eq!("false", emitter.str());

    emitter = JsonEmitter::default();
    write_json_emitter_bool(&mut emitter, true);
    assert_eq!("true", emitter.str());
}
