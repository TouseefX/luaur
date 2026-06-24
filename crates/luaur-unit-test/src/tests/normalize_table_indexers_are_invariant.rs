//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:228:normalize_table_indexers_are_invariant`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item normalize_table_indexers_are_invariant

#[cfg(test)]
#[test]
fn normalize_table_indexers_are_invariant() {
    use crate::records::is_subtype_fixture::IsSubtypeFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = IsSubtypeFixture::default();

    fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: {[string]: number}
        local b: {[string]: any}
        local c: {[string]: number}
    "#,
        ),
        None,
    );

    let a = fixture.base.require_type_string(&String::from("a"));
    let b = fixture.base.require_type_string(&String::from("b"));
    let c = fixture.base.require_type_string(&String::from("c"));

    assert!(!fixture.is_subtype(b, a));
    assert!(!fixture.is_subtype(a, b));

    assert!(fixture.is_subtype(c, a));
    assert!(fixture.is_subtype(a, c));
}
