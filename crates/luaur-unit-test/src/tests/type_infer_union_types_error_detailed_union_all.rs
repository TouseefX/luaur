//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_error_detailed_union_all() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type X = { x: number }
        type Y = { y: number }
        type Z = { z: number }

        type XYZ = X | Y | Z

        local a: XYZ = { w = 4 }
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let expected = "Expected this to be 'X | Y | Z', but got '{ w: number }'; \n\
this is because \n\
\t * the 1st component of the union is `X`, and `{ w: number }` is not a subtype of `X`\n\
\t * the 2nd component of the union is `Y`, and `{ w: number }` is not a subtype of `Y`\n\
\t * the 3rd component of the union is `Z`, and `{ w: number }` is not a subtype of `Z`\n";
        crate::CHECK_LONG_STRINGS_EQ!(expected, to_string_type_error(&result.errors[0]));
    } else {
        assert_eq!(
            "Expected this to be 'X | Y | Z', but got 'a'; none of the union options are compatible",
            to_string_type_error(&result.errors[0])
        );
    }
}
