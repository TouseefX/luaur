//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2602:type_infer_functions_tf_suggest_arg_type`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CannotInferBinaryOperation (Analysis/include/Luau/Error.h)
//!   - type_ref -> record ExplicitFunctionAnnotationRecommended (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_tf_suggest_arg_type

#[cfg(test)]
#[test]
fn type_infer_functions_tf_suggest_arg_type() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::cannot_infer_binary_operation::CannotInferBinaryOperation;
    use luaur_analysis::records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function fib(n, u)
            return (n or u) and (n < u and n + fib(n,u))
        end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<CannotInferBinaryOperation>(&result.errors[0])
        .expect("expected CannotInferBinaryOperation");
    let err = type_error_data_ref::<ExplicitFunctionAnnotationRecommended>(&result.errors[1])
        .expect("expected ExplicitFunctionAnnotationRecommended");
    assert_eq!("number", to_string_type_id(err.recommendedReturn()));
    assert_eq!(2, err.recommendedArgs().len());
    assert_eq!("number", to_string_type_id(err.recommendedArgs()[0].1));
    assert_eq!("number", to_string_type_id(err.recommendedArgs()[1].1));
}
