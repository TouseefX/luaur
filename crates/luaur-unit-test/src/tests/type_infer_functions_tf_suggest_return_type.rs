#[cfg(test)]
#[test]
fn type_infer_functions_tf_suggest_return_type() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function fib(n)
            return n < 2 and 1 or fib(n-1) + fib(n-2)
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<ExplicitFunctionAnnotationRecommended>(
        result.errors.last().expect("expected an error"),
    )
    .expect("expected ExplicitFunctionAnnotationRecommended");
    assert_eq!("false | number", to_string_type_id(err.recommendedReturn()));
}
