//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_intersect_saturate_overloaded_functions() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(x: ((number?) -> number?) & ((string?) -> string?))
            local y : (nil) -> nil = x -- Not OK (fixed in DCR)
            local z : (number) -> number = x -- Not OK
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(result.errors.len() >= 2, "{:?}", result.errors);
        let expected1 = concat!(
            "Expected this to be\n",
            "\t'(nil) -> nil'\n",
            "but got\n",
            "\t'((number?) -> number?) & ((string?) -> string?)'; \n",
            "this is because \n",
            "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the union as `number` and it returns the 1st entry in the type pack is `nil`, and `number` is not a subtype of `nil`\n",
            "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the union as `string` and it returns the 1st entry in the type pack is `nil`, and `string` is not a subtype of `nil`"
        );
        let expected2 = concat!(
            "Expected this to be\n",
            "\t'(number) -> number'\n",
            "but got\n",
            "\t'((number?) -> number?) & ((string?) -> string?)';\n",
            "this is because\n",
            "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the union as `nil` and it returns the 1st entry in the type pack is `number`, and `nil` is not a subtype of `number`\n",
            "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the union as `string` and it returns the 1st entry in the type pack is `number`, and `string` is not a subtype of `number`\n",
            "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the union as `nil` and it returns the 1st entry in the type pack is `number`, and `nil` is not a subtype of `number`\n",
            "\t * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which has the 1st component of the union as `string` and it takes the 1st entry in the type pack is `number`, and `string` is not a supertype of `number`\n",
            "\t * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which has the 2nd component of the union as `nil` and it takes the 1st entry in the type pack is `number`, and `nil` is not a supertype of `number`\n"
        );

        crate::CHECK_LONG_STRINGS_EQ!(expected1, to_string_type_error(&result.errors[0]));
        crate::CHECK_LONG_STRINGS_EQ!(expected2, to_string_type_error(&result.errors[1]));
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        let expected = r#"Expected this to be
	'(number) -> number'
but got
	'((number?) -> number?) & ((string?) -> string?)'; none of the intersection parts are compatible"#;
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
