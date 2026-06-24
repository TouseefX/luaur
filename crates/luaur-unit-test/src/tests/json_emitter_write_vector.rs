//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:101:json_emitter_write_vector`
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
//!   - translates_to -> rust_item json_emitter_write_vector

#[cfg(test)]
#[test]
fn json_emitter_write_vector() {
    use luaur_analysis::functions::write_json_emitter::write_json_emitter_vector_t;
    use luaur_analysis::records::json_emitter::JsonEmitter;

    let values = vec![1, 2, 3, 4];
    let mut emitter = JsonEmitter::default();
    write_json_emitter_vector_t(&mut emitter, &values);
    assert_eq!("[1,2,3,4]", emitter.str());
}
