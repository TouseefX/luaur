//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1677:type_function_fully_dispatch_type_function_that_is_parameterized_on_a_stuck_type_function`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> record ConstraintSolvingIncompleteError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_function_fully_dispatch_type_function_that_is_parameterized_on_a_stuck_type_function

#[cfg(test)]
#[test]
fn type_function_fully_dispatch_type_function_that_is_parameterized_on_a_stuck_type_function() {
    use crate::functions::has_error::has_error;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        local function f()
            local a
            local b

            local c = a + b

            print(c + d)
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert!(
        !has_error::<ConstraintSolvingIncompleteError>(&result),
        "{:?}",
        result.errors
    );
    assert_eq!(
        "() -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
}
