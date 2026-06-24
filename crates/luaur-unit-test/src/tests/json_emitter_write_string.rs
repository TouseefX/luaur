//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:52:json_emitter_write_string`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item json_emitter_write_string

#[cfg(test)]
#[test]
fn json_emitter_write_string() {
    use luaur_analysis::functions::write_json_emitter_alt_ae::write_json_emitter_string_view;
    use luaur_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    write_json_emitter_string_view(
        &mut emitter,
        r#"foo,bar,baz,
"this should be escaped""#,
    );
    assert_eq!(
        "\"foo,bar,baz,\\n\\\"this should be escaped\\\"\"",
        emitter.str()
    );
}
