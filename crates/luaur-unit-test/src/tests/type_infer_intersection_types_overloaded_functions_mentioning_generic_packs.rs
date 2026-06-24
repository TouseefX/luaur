//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_overloaded_functions_mentioning_generic_packs() {
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
        function f<a...,b...>()
            function g(x : ((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...)))
                local y : ((nil, a...) -> (nil, b...)) = x -- OK in the old solver, not OK in the new
                local z : ((nil, b...) -> (nil, a...)) = x -- Not OK
                local w : ((number?, a...) -> (number?, b...)) = x -- OK in both solvers
            end
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        let tm1 = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
            .expect("expected TypeMismatch");
        assert_eq!(
            "(nil, a...) -> (nil, b...)",
            to_string_type_id(tm1.wanted_type)
        );
        assert_eq!(
            "((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...))",
            to_string_type_id(tm1.given_type)
        );
        let tm2 = unsafe { get_type_error::<TypeMismatch>(&result.errors[1]).as_ref() }
            .expect("expected TypeMismatch");
        assert_eq!(
            "(nil, b...) -> (nil, a...)",
            to_string_type_id(tm2.wanted_type)
        );
        assert_eq!(
            "((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...))",
            to_string_type_id(tm2.given_type)
        );

        let expected1 = concat!(
            "Expected this to be\n\t",
            "'(nil, a...) -> (nil, b...)'",
            "\nbut got\n\t",
            "'((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...))'",
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
            "'(nil, b...) -> (nil, a...)'",
            "\nbut got\n\t",
            "'((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...))'",
            "; \nthis is because \n\t",
            " * in the 1st component of the intersection, the function returns a tail of `b...` and it returns a tail of `a...`, and `b...` is ",
            "not a ",
            "subtype of `a...`\n\t",
            " * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
            "the ",
            "union as `number` and it returns the 1st entry in the type pack is `nil`, and `number` is not a subtype of `nil`\n\t",
            " * in the 1st component of the intersection, the function takes a tail of `a...` and it takes a tail of `b...`, and `a...` is not ",
            "a ",
            "supertype of `b...`\n\t",
            " * in the 2nd component of the intersection, the function returns a tail of `b...` and it returns a tail of `a...`, and `b...` is ",
            "not a ",
            "subtype of `a...`\n\t",
            " * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
            "the ",
            "union as `string` and it returns the 1st entry in the type pack is `nil`, and `string` is not a subtype of `nil`\n\t",
            " * in the 2nd component of the intersection, the function takes a tail of `a...` and it takes a tail of `b...`, and `a...` is not ",
            "a ",
            "supertype of `b...`"
        );

        assert_eq!(expected1, to_string_type_error(&result.errors[0]));
        assert_eq!(expected2, to_string_type_error(&result.errors[1]));
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        let expected = r#"Expected this to be
	'(nil, b...) -> (nil, a...)'
but got
	'((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...))'; none of the intersection parts are compatible"#;
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
