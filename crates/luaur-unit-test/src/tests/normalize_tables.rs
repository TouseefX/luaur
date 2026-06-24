//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:195:normalize_tables`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item normalize_tables

#[cfg(test)]
#[test]
fn normalize_tables() {
    use crate::records::is_subtype_fixture::IsSubtypeFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    let mut fixture = IsSubtypeFixture::default();

    fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: {x: number}
        local b: {x: any}
        local c: {y: number}
        local d: {x: number, y: number}
    "#,
        ),
        None,
    );

    let a = fixture.base.require_type_string(&String::from("a"));
    let b = fixture.base.require_type_string(&String::from("b"));
    let c = fixture.base.require_type_string(&String::from("c"));
    let d = fixture.base.require_type_string(&String::from("d"));

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(!fixture.is_subtype(a, b));
    } else {
        assert!(fixture.is_subtype(a, b));
    }
    assert!(!fixture.is_subtype(b, a));

    assert!(!fixture.is_subtype(c, a));
    assert!(!fixture.is_subtype(a, c));

    assert!(fixture.is_subtype(d, a));
    assert!(!fixture.is_subtype(a, d));

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(!fixture.is_subtype(d, b));
    } else {
        assert!(fixture.is_subtype(d, b));
    }
    assert!(!fixture.is_subtype(b, d));
}
