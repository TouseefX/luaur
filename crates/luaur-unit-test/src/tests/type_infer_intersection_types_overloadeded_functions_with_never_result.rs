//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_overloadeded_functions_with_never_result() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
    function f<a...,b...>()
        function g(x : ((number) -> number) & ((nil) -> never))
            local y : (number?) -> number = x -- OK
            local z : (number?) -> never = x -- Not OK
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
            "'(number?) -> number'",
            "\nbut got\n\t",
            "'((nil) -> never) & ((number) -> number)'",
            "; \nthis is because \n\t",
            " * in the 1st component of the intersection, the function takes the 1st entry in the type pack which is `number` and it takes the ",
            "1st ",
            "entry in the type pack has the 2nd component of the union as `nil`, and `number` is not a supertype of `nil`\n\t",
            " * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which is `nil` and it takes the ",
            "1st ",
            "entry in the type pack has the 1st component of the union as `number`, and `nil` is not a supertype of `number`"
        );
        let expected2 = concat!(
            "Expected this to be\n\t",
            "'(number?) -> never'",
            "\nbut got\n\t",
            "'((nil) -> never) & ((number) -> number)'",
            "; \nthis is because \n\t",
            " * in the 1st component of the intersection, the function returns the 1st entry in the type pack which is `number` and it returns ",
            "the ",
            "1st entry in the type pack is `never`, and `number` is not a subtype of `never`\n\t",
            " * in the 1st component of the intersection, the function takes the 1st entry in the type pack which is `number` and it takes the ",
            "1st ",
            "entry in the type pack has the 2nd component of the union as `nil`, and `number` is not a supertype of `nil`\n\t",
            " * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which is `nil` and it takes the ",
            "1st ",
            "entry in the type pack has the 1st component of the union as `number`, and `nil` is not a supertype of `number`"
        );

        assert_eq!(expected1, to_string_type_error(&result.errors[0]));
        assert_eq!(expected2, to_string_type_error(&result.errors[1]));
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        let expected = r#"Expected this to be
	'(number?) -> never'
but got
	'((nil) -> never) & ((number) -> number)'; none of the intersection parts are compatible"#;
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
