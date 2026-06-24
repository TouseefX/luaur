//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:67:nonstrict_mode_return_annotation_is_still_checked`
//! Source: `tests/NonstrictMode.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NonstrictMode.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/NonstrictMode.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item nonstrict_mode_return_annotation_is_still_checked

#[cfg(test)]
#[test]
fn nonstrict_mode_return_annotation_is_still_checked() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(x): number return 'hello' end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_ne!(
        "any",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
}
