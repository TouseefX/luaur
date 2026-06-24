//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_overloaded_functions_mentioning_generic() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a>()
            function g(x : ((number?) -> (a | number)) & ((string?) -> (a | string)))
                local y : (nil) -> a = x -- OK
                local z : (number?) -> a = x -- Not OK
            end
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        let expected = r#"Expected this to be
	'(number?) -> a'
but got
	'((number?) -> a | number) & ((string?) -> a | string)'; none of the intersection parts are compatible"#;
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
