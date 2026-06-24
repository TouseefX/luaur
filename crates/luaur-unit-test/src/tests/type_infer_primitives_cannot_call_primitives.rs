//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:14:type_infer_primitives_cannot_call_primitives`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CannotCallNonFunction (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_primitives_cannot_call_primitives

#[cfg(test)]
#[test]
fn type_infer_primitives_cannot_call_primitives() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture
        .check_string_optional_frontend_options(&String::from("local foo = 5    foo()"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
        matches!(
            &result.errors[0].data,
            TypeErrorData::CannotCallNonFunction(_)
        ),
        "{:?}",
        result.errors[0]
    );
}
