//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:173:type_infer_singletons_overloaded_function_call_with_singletons_mismatch`
//! Source: `tests/TypeInfer.singletons.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.singletons.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.singletons.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_singletons_overloaded_function_call_with_singletons_mismatch

#[cfg(test)]
#[test]
fn type_infer_singletons_overloaded_function_call_with_singletons_mismatch() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(g: ((true, string) -> ()) & ((false, number) -> ()))
            g(true, 37)
        end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "None of the overloads for function that accept 2 arguments are compatible.",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Available overloads: (true, string) -> (); and (false, number) -> ()",
            to_string_type_error(&result.errors[1])
        );
    } else {
        assert_eq!(
            "Expected this to be 'string', but got 'number'",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Other overloads are also not viable: (false, number) -> ()",
            to_string_type_error(&result.errors[1])
        );
    }
}
