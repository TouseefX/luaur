//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:120:json_emitter_prevent_multiple_array_finish`
//! Source: `tests/JsonEmitter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/JsonEmitter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
//! - incoming:
//!   - declares <- source_file tests/JsonEmitter.test.cpp
//! - outgoing:
//!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
//!   - type_ref -> record ArrayEmitter (Analysis/include/Luau/JsonEmitter.h)
//!   - calls -> method JsonEmitter::writeArray (Analysis/src/JsonEmitter.cpp)
//!   - calls -> method ArrayEmitter::writeValue (Analysis/include/Luau/JsonEmitter.h)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item json_emitter_prevent_multiple_array_finish

#[cfg(test)]
#[test]
fn json_emitter_prevent_multiple_array_finish() {
    use luaur_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    let mut a = emitter.write_array();
    a.write_value(1);
    a.finish();
    a.finish();

    assert_eq!("[1]", emitter.str());
}
