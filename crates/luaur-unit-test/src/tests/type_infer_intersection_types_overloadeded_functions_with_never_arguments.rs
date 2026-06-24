//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_overloadeded_functions_with_never_arguments() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a...,b...>()
            function g(x : ((number) -> number?) & ((never) -> string?))
                local y : (never) -> nil = x -- OK
                local z : (number?) -> nil = x -- Not OK
            end
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(result.errors.len() >= 2, "{:?}", result.errors);
        let expected1 = concat!(
            "Expected this to be\n\t",
            "'(never) -> nil'",
            "\nbut got\n\t",
            "'((never) -> string?) & ((number) -> number?)'",
            "; \nthis is because \n\t",
            " * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
            "the ",
            "union as `number` and it returns the 1st entry in the type pack is `nil`, and `number` is not a subtype of `nil`\n\t",
            " * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
            "the ",
            "union as `string` and it returns the 1st entry in the type pack is `nil`, and `string` is not a subtype of `nil`"
        );
        let expected2 = concat!(
            "Expected this to be\n\t",
            "'(number?) -> nil'",
            "\nbut got\n\t",
            "'((never) -> string?) & ((number) -> number?)'",
            "; \nthis is because \n\t",
            " * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
            "the ",
            "union as `number` and it returns the 1st entry in the type pack is `nil`, and `number` is not a subtype of `nil`\n\t",
            " * in the 1st component of the intersection, the function takes the 1st entry in the type pack which is `number` and it takes the ",
            "1st ",
            "entry in the type pack has the 2nd component of the union as `nil`, and `number` is not a supertype of `nil`\n\t",
            " * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
            "the ",
            "union as `string` and it returns the 1st entry in the type pack is `nil`, and `string` is not a subtype of `nil`\n\t",
            " * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which is `never` and it takes the ",
            "1st ",
            "entry in the type pack has the 1st component of the union as `number`, and `never` is not a supertype of `number`\n\t",
            " * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which is `never` and it takes the ",
            "1st ",
            "entry in the type pack has the 2nd component of the union as `nil`, and `never` is not a supertype of `nil`"
        );

        assert_eq!(expected1, to_string_type_error(&result.errors[0]));
        assert_eq!(expected2, to_string_type_error(&result.errors[1]));
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        let expected = r#"Expected this to be
	'(number?) -> nil'
but got
	'((never) -> string?) & ((number) -> number?)'; none of the intersection parts are compatible"#;
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
