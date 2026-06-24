//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_union_of_functions_mentioning_generic_typepacks() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a...>()
            function g(x : (number, a...) -> (number?, a...))
                local y : ((number | string, a...) -> (number, a...)) | ((number?, a...) -> (nil, a...)) = x -- OK
                local z : ((number) -> number) | ((number?, a...) -> (number?, a...)) = x -- Not OK
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t\
'((number) -> number) | ((number?, a...) -> (number?, a...))'\
\nbut got\n\t\
'(number, a...) -> (number?, a...)'\
; none of the union options are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
