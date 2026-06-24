//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:440:type_infer_type_packs_type_alias_type_pack_variadic`
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
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_pack_variadic

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_type_pack_variadic() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type X<T...> = (T...) -> (string, T...)

type D = X<...number>
type E = X<(number, ...string)>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let d = fixture
        .lookup_type(&String::from("D"))
        .expect("expected type alias D");
    let e = fixture
        .lookup_type(&String::from("E"))
        .expect("expected type alias E");
    assert_eq!("(...number) -> (string, ...number)", to_string_type_id(d));
    assert_eq!(
        "(number, ...string) -> (string, number, ...string)",
        to_string_type_id(e)
    );
}
