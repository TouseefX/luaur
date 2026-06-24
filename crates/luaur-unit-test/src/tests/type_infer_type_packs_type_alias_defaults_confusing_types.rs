//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:892:type_infer_type_packs_type_alias_defaults_confusing_types`
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
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_defaults_confusing_types

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_defaults_confusing_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type A<T, U = T, V... = ...any, W... = V...> = (T, V...) -> (U, W...)
type B = A<string, (number)>
type C = A<string, (number), (boolean)>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let b = fixture
        .lookup_type(&String::from("B"))
        .expect("expected type alias B");
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "(string, ...any) -> (number, ...any)",
        to_string_type_id_to_string_options(b, &mut opts)
    );

    let c = fixture
        .lookup_type(&String::from("C"))
        .expect("expected type alias C");
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "(string, boolean) -> (number, boolean)",
        to_string_type_id_to_string_options(c, &mut opts)
    );
}
