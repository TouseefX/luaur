//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:420:type_infer_type_packs_type_alias_type_packs_nested`
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
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_packs_nested

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_type_packs_nested() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Packed1<T...> = (T...) -> (T...)
type Packed2<T...> = (Packed1<T...>, T...) -> (Packed1<T...>, T...)
type Packed3<T...> = (Packed2<T...>, T...) -> (Packed2<T...>, T...)
type Packed4<T...> = (Packed3<T...>, T...) -> (Packed3<T...>, T...)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let packed = fixture
        .lookup_type(&String::from("Packed4"))
        .expect("expected type alias Packed4");
    assert_eq!(
        "((((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...) -> (((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...), T...) -> \
((((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...) -> (((T...) -> (T...), T...) -> ((T...) -> (T...), T...), T...), T...)",
        to_string_type_id(packed)
    );
}
