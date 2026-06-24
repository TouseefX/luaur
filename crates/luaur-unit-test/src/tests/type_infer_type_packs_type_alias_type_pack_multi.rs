//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:459:type_infer_type_packs_type_alias_type_pack_multi`
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
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_pack_multi

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_type_pack_multi() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Y<T..., U...> = (T...) -> (U...)
type A<S...> = Y<S..., S...>
type B<S...> = Y<(number, ...string), S...>

type Z<T, U...> = (T) -> (U...)
type E<S...> = Z<number, S...>
type F<S...> = Z<number, (string, S...)>

type W<T, U..., V...> = (T, U...) -> (T, V...)
type H<S..., R...> = W<number, S..., R...>
type I<S..., R...> = W<number, (string, S...), R...>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    for (name, expected) in [
        ("A", "(S...) -> (S...)"),
        ("B", "(number, ...string) -> (S...)"),
        ("E", "(number) -> (S...)"),
        ("F", "(number) -> (string, S...)"),
        ("H", "(number, S...) -> (number, R...)"),
        ("I", "(number, string, S...) -> (number, R...)"),
    ] {
        let ty = fixture
            .lookup_type(&String::from(name))
            .unwrap_or_else(|| panic!("expected type alias {name}"));
        assert_eq!(expected, to_string_type_id(ty), "{name}");
    }
}
