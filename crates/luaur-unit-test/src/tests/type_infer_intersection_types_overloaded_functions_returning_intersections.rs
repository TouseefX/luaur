//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_overloaded_functions_returning_intersections() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x : ((number?) -> ({ p : number } & { q : number })) & ((string?) -> ({ p : number } & { r : number })))
            local y : (nil) -> { p : number, q : number, r : number} = x -- OK
            local z : (number?) -> { p : number, q : number, r : number} = x -- Not OK
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        let expected1 = concat!(
            "Expected this to be\n",
            "\t'(nil) -> { p: number, q: number, r: number }'\n",
            "but got\n",
            "\t'((number?) -> { p: number } & { q: number }) & ((string?) -> { p: number } & { r: number })'; \n",
            "this is because \n",
            "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the intersection as `{ p: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ p: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
            "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the intersection as `{ q: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ q: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
            "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the intersection as `{ p: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ p: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
            "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the intersection as `{ r: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ r: number }` is not a subtype of `{ p: number, q: number, r: number }`"
        );
        let expected2 = concat!(
            "Expected this to be\n",
            "\t'(number?) -> { p: number, q: number, r: number }'\n",
            "but got\n",
            "\t'((number?) -> { p: number } & { q: number }) & ((string?) -> { p: number } & { r: number })'; \n",
            "this is because \n",
            "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the intersection as `{ p: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ p: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
            "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the intersection as `{ q: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ q: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
            "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the intersection as `{ p: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ p: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
            "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the intersection as `{ r: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ r: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
            "\t * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which has the 1st component of the union as `string` and it takes the 1st entry in the type pack has the 1st component of the union as `number`, and `string` is not a supertype of `number`\n",
            "\t * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which has the 2nd component of the union as `nil` and it takes the 1st entry in the type pack has the 1st component of the union as `number`, and `nil` is not a supertype of `number`"
        );

        assert_eq!(expected1, to_string_type_error(&result.errors[0]));
        assert_eq!(expected2, to_string_type_error(&result.errors[1]));
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        let expected = r#"Expected this to be
	'(number?) -> { p: number, q: number, r: number }'
but got
	'((number?) -> { p: number } & { q: number }) & ((string?) -> { p: number } & { r: number })'; none of the intersection parts are compatible"#;
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
