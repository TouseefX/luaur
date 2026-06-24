//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_union_of_functions_mentioning_generics() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a,b>()
            function g(x : (a) -> a?)
                local y : ((a?) -> nil) | ((a) -> a) = x -- OK
                local z : ((b?) -> nil) | ((b) -> b) = x -- Not OK
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Expected this to be '((b) -> b) | ((b?) -> nil)', but got '(a) -> a?'; none of the union options are compatible",
        to_string_type_error(&result.errors[0])
    );
}
