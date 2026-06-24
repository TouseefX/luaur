//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_error_detailed_intersection_part() {
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
type XYZ = X & Y & Z
local a: XYZ = 3
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        concat!(
            "Expected this to be 'X & Y & Z', but got 'number'; \n",
            "this is because \n\t",
            " * the 1st component of the intersection is `X`, and `number` is not a subtype of `X`\n\t",
            " * the 2nd component of the intersection is `Y`, and `number` is not a subtype of `Y`\n\t",
            " * the 3rd component of the intersection is `Z`, and `number` is not a subtype of `Z`"
        )
    } else {
        r#"Expected this to be 'X & Y & Z', but got 'number'
caused by:
  Not all intersection parts are compatible.
Expected this to be 'X', but got 'number'"#
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
