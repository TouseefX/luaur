//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:22:type_infer_primitives_string_length`
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
//!   - translates_to -> rust_item type_infer_primitives_string_length

#[cfg(test)]
#[test]
fn type_infer_primitives_string_length() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local s = "Hello, World!"
        local t = #s
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let number_type = fixture.get_builtins().numberType;
    assert_eq!(number_type, fixture.require_type_string(&String::from("t")));
}
