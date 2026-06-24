//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_intersect_false_and_bool_and_false() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x: false & (boolean & false))
            local y : false = x -- OK
            local z : true = x  -- Not OK
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        concat!(
            "Expected this to be 'true', but got 'boolean & false & false'; \n",
            "this is because \n\t",
            " * the 1st component of the intersection is `false`, which is not a subtype of `true`\n\t",
            " * the 2nd component of the intersection is `boolean`, which is not a subtype of `true`\n\t",
            " * the 3rd component of the intersection is `false`, which is not a subtype of `true`"
        )
    } else {
        "Expected this to be 'true', but got 'boolean & false & false'; none of the intersection parts are compatible"
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
