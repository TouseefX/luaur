//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:562:type_infer_type_packs_type_alias_type_packs_errors`
//! Source: `tests/TypeInfer.typePacks.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_packs_errors

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_type_packs_errors() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = Fixture::fixture_bool(false);
    let mut check_error = |source: &str, expected: &str| {
        let result = fixture.check_string_optional_frontend_options(&String::from(source), None);
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    };

    check_error(
        r#"
type Packed<T, U, V...> = (T, U) -> (V...)
local b: Packed<number>
    "#,
        "Generic type 'Packed<T, U, V...>' expects at least 2 type arguments, but only 1 is specified",
    );

    check_error(
        r#"
type Packed<T, U> = (T, U) -> ()
type B<X...> = Packed<number, string, X...>
    "#,
        "Generic type 'Packed<T, U>' expects 0 type pack arguments, but 1 is specified",
    );

    check_error(
        r#"
type Packed<T..., U...> = (T...) -> (U...)
type Other<S...> = Packed<S..., string>
    "#,
        "Type parameters must come before type pack parameters",
    );

    check_error(
        r#"
type Packed<T, U> = (T) -> U
type Other<S...> = Packed<number, S...>
    "#,
        "Generic type 'Packed<T, U>' expects 2 type arguments, but only 1 is specified",
    );

    check_error(
        r#"
type Packed<T..., U...> = (T...) -> (U...)
type Other = Packed<>
    "#,
        "Generic type 'Packed<T..., U...>' expects 2 type pack arguments, but none are specified",
    );

    check_error(
        r#"
type Packed<T..., U...> = (T...) -> (U...)
type Other = Packed<number, string>
    "#,
        "Generic type 'Packed<T..., U...>' expects 2 type pack arguments, but only 1 is specified",
    );
}
