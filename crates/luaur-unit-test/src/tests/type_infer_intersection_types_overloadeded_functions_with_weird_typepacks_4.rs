//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_overloadeded_functions_with_weird_typepacks_4() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a...>()
            function g(x : ((a...) -> ()) & ((number,a...) -> number))
                local y : ((number,a...) -> number) & ((a...) -> ()) = x -- OK
                local z : (number?) -> () = x -- Not OK
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let err = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
            .expect("expected TypeMismatch");
        assert_eq!("(number?) -> ()", to_string_type_id(err.wanted_type));
        assert_eq!(
            "((a...) -> ()) & ((number, a...) -> number)",
            to_string_type_id(err.given_type)
        );
        let expected = concat!(
            "Expected this to be\n\t",
            "'(number?) -> ()'",
            "\nbut got\n\t",
            "'((a...) -> ()) & ((number, a...) -> number)'",
            "; \nthis is because \n\t",
            " * in the 1st component of the intersection, the function takes a tail of `a...` and it takes the portion of the type pack starting at ",
            "index 0 to the end`number?`, and `a...` is not a supertype of `number?`\n\t",
            " * in the 2nd component of the intersection, the function returns is `number` and it returns `()`, and `number` is not a subtype of ",
            "`()`\n\t",
            " * in the 2nd component of the intersection, the function takes a tail of `a...` and it takes `number?`, and `a...` is not a ",
            "supertype ",
            "of `number?`\n\t",
            " * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which is `number` and it takes the ",
            "1st ",
            "entry in the type pack has the 2nd component of the union as `nil`, and `number` is not a supertype of `nil`"
        );
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
        let expected = r#"Expected this to be
	'(number?) -> ()'
but got
	'((a...) -> ()) & ((number, a...) -> number)'; none of the intersection parts are compatible"#;
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
