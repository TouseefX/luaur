//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:156:normalize_intersection`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item normalize_intersection

#[cfg(test)]
#[test]
fn normalize_intersection() {
    use crate::records::is_subtype_fixture::IsSubtypeFixture;
    use alloc::string::String;

    let mut fixture = IsSubtypeFixture::default();

    fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: number & string
        local b: number
        local c: string
        local d: number & nil
    "#,
        ),
        None,
    );

    let a = fixture.base.require_type_string(&String::from("a"));
    let b = fixture.base.require_type_string(&String::from("b"));
    let c = fixture.base.require_type_string(&String::from("c"));
    let d = fixture.base.require_type_string(&String::from("d"));

    assert!(!fixture.is_subtype(b, a));
    assert!(fixture.is_subtype(a, b));

    assert!(!fixture.is_subtype(c, a));
    assert!(fixture.is_subtype(a, c));

    assert!(fixture.is_subtype(d, a));
    assert!(fixture.is_subtype(a, d));
}
