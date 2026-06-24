//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:234:type_infer_unknownnever_assign_to_subscript_which_is_never`
//! Source: `tests/TypeInfer.unknownnever.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_unknownnever_assign_to_subscript_which_is_never

#[cfg(test)]
#[test]
fn type_infer_unknownnever_assign_to_subscript_which_is_never() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(t: never)
            t[5] = 7
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
