//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:185:json_emitter_afford_extensibility`
//! Source: `tests/JsonEmitter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/JsonEmitter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
//! - incoming:
//!   - declares <- source_file tests/JsonEmitter.test.cpp
//! - outgoing:
//!   - type_ref -> record Special (tests/JsonEmitter.test.cpp)
//!   - calls -> type_alias vec (Common/include/Luau/InsertionOrderedMap.h)
//!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item json_emitter_afford_extensibility

#[cfg(test)]
#[test]
fn json_emitter_afford_extensibility() {
    use crate::records::special::Special;
    use luaur_analysis::functions::write_json_emitter::write_json_emitter_vector_t;
    use luaur_analysis::records::json_emitter::JsonEmitter;

    let vec = vec![Special { foo: 1, bar: 2 }, Special { foo: 3, bar: 4 }];
    let mut e = JsonEmitter::default();
    write_json_emitter_vector_t(&mut e, &vec);

    let result = e.str();
    assert_eq!("[{\"foo\":1,\"bar\":2},{\"foo\":3,\"bar\":4}]", result);
}
