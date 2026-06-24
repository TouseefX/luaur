//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_overloadeded_functions_with_unknown_result() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a...,b...>()
            function g(x : ((number) -> number) & ((nil) -> unknown))
                local y : (number?) -> unknown = x -- OK
                local z : (number?) -> number? = x -- Not OK
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = concat!(
        "Expected this to be\n\t",
        "'(number?) -> number?'",
        "\nbut got\n\t",
        "'((nil) -> unknown) & ((number) -> number)'",
        "; none of the intersection parts are compatible"
    );
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
