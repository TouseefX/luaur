//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:49:type_infer_primitives_string_method`
//! Source: `tests/TypeInfer.primitives.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_primitives_string_method

#[cfg(test)]
#[test]
fn type_infer_primitives_string_method() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local p = ("tacos"):len()
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let ty = fixture.require_type_string(&String::from("p"));
    assert_eq!("number", to_string_type_id(ty));
}
