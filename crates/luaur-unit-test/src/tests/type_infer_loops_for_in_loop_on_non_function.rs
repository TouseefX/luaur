//! Ported from `tests/TypeInfer.loops.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:319:type_infer_loops_for_in_loop_on_non_function`
//! Source: `tests/TypeInfer.loops.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.loops.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.loops.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CannotCallNonFunction (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_loops_for_in_loop_on_non_function

#[cfg(test)]
#[test]
fn type_infer_loops_for_in_loop_on_non_function() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::cannot_call_non_function::CannotCallNonFunction;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local bad_iter = 5

        for a in bad_iter() do
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<CannotCallNonFunction>(&result.errors[0])
        .expect("expected CannotCallNonFunction");
}
