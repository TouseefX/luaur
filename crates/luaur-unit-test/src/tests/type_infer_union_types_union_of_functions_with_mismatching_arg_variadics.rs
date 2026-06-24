//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_union_of_functions_with_mismatching_arg_variadics() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x : (number) -> ())
            local y : ((number?) -> ()) | ((...number) -> ()) = x -- OK
            local z : ((number?) -> ()) | ((...number?) -> ()) = x -- Not OK
        end
     "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        let expected = "Expected this to be\n\
\t'((...number?) -> ()) | ((number?) -> ())'\n\
but got\n\
\t'(number) -> ()'; \n\
this is because \n\
\t * it takes `number` and in the 2nd component of the union, the function takes a tail of `...number?`, and `number` is not a supertype of `...number?`\n\
\t * it takes the 1st entry in the type pack is `number` and in the 1st component of the union, the function takes the 1st entry in the type pack which has the 2nd component of the union as `nil`, and `number` is not a supertype of `nil`";
        crate::CHECK_LONG_STRINGS_EQ!(expected, to_string_type_error(&result.errors[0]));
    } else {
        let expected = "Expected this to be\n\t\
'((...number?) -> ()) | ((number?) -> ())'\
\nbut got\n\t\
'(number) -> ()'; none of the union options are compatible";
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
