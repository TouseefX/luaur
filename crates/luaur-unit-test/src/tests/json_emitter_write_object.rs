//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:22:json_emitter_write_object`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item json_emitter_write_object

#[cfg(test)]
#[test]
fn json_emitter_write_object() {
    use luaur_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    let mut o = emitter.write_object();
    o.write_pair("foo", "bar");
    o.write_pair("bar", "baz");
    o.finish();

    let result = emitter.str();
    assert_eq!("{\"foo\":\"bar\",\"bar\":\"baz\"}", result);
}
