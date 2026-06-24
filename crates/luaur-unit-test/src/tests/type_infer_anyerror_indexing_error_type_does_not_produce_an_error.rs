//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.anyerror.test.cpp:176:type_infer_anyerror_indexing_error_type_does_not_produce_an_error`
//! Source: `tests/TypeInfer.anyerror.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.anyerror.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.anyerror.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_anyerror_indexing_error_type_does_not_produce_an_error

#[cfg(test)]
#[test]
fn type_infer_anyerror_indexing_error_type_does_not_produce_an_error() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local originalReward = unknown.Parent.Reward:GetChildren()[1]
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}
