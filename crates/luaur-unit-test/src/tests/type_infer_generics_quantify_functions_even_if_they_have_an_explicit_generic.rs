//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1652:type_infer_generics_quantify_functions_even_if_they_have_an_explicit_generic`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_generics_quantify_functions_even_if_they_have_an_explicit_generic

#[cfg(test)]
#[test]
fn type_infer_generics_quantify_functions_even_if_they_have_an_explicit_generic() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo<X>(f, x: X)
            return f(x)
        end
    "#,
        ),
        None,
    );

    assert_eq!(
        "<X, a...>((X) -> (a...), X) -> (a...)",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
}
