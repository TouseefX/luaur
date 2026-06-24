//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:75:type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_another_overload_works`
//! Source: `tests/TypeInfer.oop.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.oop.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.oop.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_another_overload_works

#[cfg(test)]
#[test]
fn type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_another_overload_works() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = {method: ((T, number) -> number) & ((number) -> number)}
        local T: T

        T.method(4)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
