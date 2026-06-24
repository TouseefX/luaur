//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:487:type_infer_type_packs_type_alias_type_pack_explicit`
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
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_pack_explicit

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_type_pack_explicit() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type X<T...> = (T...) -> (T...)

type A<S...> = X<(S...)>
type B = X<()>
type C = X<(number)>
type D = X<(number, string)>
type E = X<(...number)>
type F = X<(string, ...number)>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    for (name, expected) in [
        ("A", "(S...) -> (S...)"),
        ("B", "() -> ()"),
        ("C", "(number) -> number"),
        ("D", "(number, string) -> (number, string)"),
        ("E", "(...number) -> (...number)"),
        ("F", "(string, ...number) -> (string, ...number)"),
    ] {
        let ty = fixture
            .lookup_type(&String::from(name))
            .unwrap_or_else(|| panic!("expected type alias {name}"));
        assert_eq!(expected, to_string_type_id(ty), "{name}");
    }
}
