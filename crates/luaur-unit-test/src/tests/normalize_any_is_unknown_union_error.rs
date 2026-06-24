//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:349:normalize_any_is_unknown_union_error`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item normalize_any_is_unknown_union_error

#[cfg(test)]
#[test]
fn normalize_any_is_unknown_union_error() {
    use crate::records::is_subtype_fixture::IsSubtypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = IsSubtypeFixture::default();

    fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local err = 5.nope.nope -- err is now an error type
        local a : any
        local b : (unknown | typeof(err))
    "#,
        ),
        None,
    );

    let a = fixture.base.require_type_string(&String::from("a"));
    let b = fixture.base.require_type_string(&String::from("b"));

    assert!(fixture.is_subtype(a, b));
    assert!(fixture.is_subtype(b, a));
    assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.base.require_type_string(&String::from("err")))
    );
}
