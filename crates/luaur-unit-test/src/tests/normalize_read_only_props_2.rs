//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1019:normalize_read_only_props_2`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method NormalizeFixture::normal (tests/Normalize.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item normalize_read_only_props_2

#[cfg(test)]
#[test]
fn normalize_read_only_props_2() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = NormalizeFixture::default();

    assert_eq!(
        r#"{ x: "hello" }"#,
        to_string_type_id_to_string_options_mut(
            fixture.normal(r#"{ x: "hello" } & { x: string }"#),
            ToStringOptions::to_string_options(true)
        )
    );
    assert_eq!(
        "never",
        to_string_type_id_to_string_options_mut(
            fixture.normal(r#"{ x: "hello" } & { x: "world" }"#),
            ToStringOptions::to_string_options(true)
        )
    );
}
