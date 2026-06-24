//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:141:json_emitter_cannot_write_value_after_finished`
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
//!   - translates_to -> rust_item json_emitter_cannot_write_value_after_finished

#[cfg(test)]
#[test]
fn json_emitter_cannot_write_value_after_finished() {
    use luaur_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    let mut a = emitter.write_array();
    a.finish();
    a.write_value(1);

    assert_eq!("[]", emitter.str());
}
