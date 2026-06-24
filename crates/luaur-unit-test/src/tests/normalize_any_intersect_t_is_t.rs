//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:365:normalize_any_intersect_t_is_t`
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
//!   - translates_to -> rust_item normalize_any_intersect_t_is_t

#[cfg(test)]
#[test]
fn normalize_any_intersect_t_is_t() {
    use crate::records::is_subtype_fixture::IsSubtypeFixture;
    use alloc::string::String;

    let mut fixture = IsSubtypeFixture::default();

    fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a : (any & string)
        local b : string
        local c : number
    "#,
        ),
        None,
    );

    let a = fixture.base.require_type_string(&String::from("a"));
    let b = fixture.base.require_type_string(&String::from("b"));
    let c = fixture.base.require_type_string(&String::from("c"));

    assert!(fixture.is_subtype(a, b));
    assert!(fixture.is_subtype(b, a));
    assert!(!fixture.is_subtype(a, c));
    assert!(!fixture.is_subtype(c, a));
}
