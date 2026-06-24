//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_union_of_functions_with_mismatching_result_variadics() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x : () -> (number?, ...number))
            local y : (() -> (...number)) | (() -> nil) = x -- OK
            local z : (() -> (...number)) | (() -> number) = x -- OK
        end
     "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t\
'(() -> (...number)) | (() -> number)'\
\nbut got\n\t\
'() -> (number?, ...number)'\
; none of the union options are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
