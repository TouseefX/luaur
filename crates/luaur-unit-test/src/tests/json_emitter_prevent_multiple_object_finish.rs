//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:109:json_emitter_prevent_multiple_object_finish`
//! Source: `tests/JsonEmitter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/JsonEmitter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
//! - incoming:
//!   - declares <- source_file tests/JsonEmitter.test.cpp
//! - outgoing:
//!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
//!   - type_ref -> record ObjectEmitter (Analysis/include/Luau/JsonEmitter.h)
//!   - calls -> method JsonEmitter::writeObject (Analysis/src/JsonEmitter.cpp)
//!   - calls -> method ObjectEmitter::writePair (Analysis/include/Luau/JsonEmitter.h)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item json_emitter_prevent_multiple_object_finish

#[cfg(test)]
#[test]
fn json_emitter_prevent_multiple_object_finish() {
    use luaur_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    let mut o = emitter.write_object();
    o.write_pair("a", "b");
    o.finish();
    o.finish();

    assert_eq!("{\"a\":\"b\"}", emitter.str());
}
