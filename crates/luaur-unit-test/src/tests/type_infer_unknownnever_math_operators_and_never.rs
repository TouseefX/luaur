//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:344:type_infer_unknownnever_math_operators_and_never`
//! Source: `tests/TypeInfer.unknownnever.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record ExplicitFunctionAnnotationRecommended (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_unknownnever_math_operators_and_never

#[cfg(test)]
#[test]
fn type_infer_unknownnever_math_operators_and_never() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function mul(x: nil, y)
            return x ~= nil and x * y -- infers boolean | never, which is normalized into boolean
        end
    "#,
        ),
        None,
    );

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert!(
            matches!(
                result.errors[0].data,
                TypeErrorData::ExplicitFunctionAnnotationRecommended(_)
            ),
            "{:?}",
            result.errors[0]
        );

        assert_eq!(
            "<a>(nil, a) -> false | mul<nil & ~nil, a>",
            to_string_type_id(fixture.require_type_string(&String::from("mul")))
        );
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "<a>(nil, a) -> boolean",
            to_string_type_id(fixture.require_type_string(&String::from("mul")))
        );
    }
}
