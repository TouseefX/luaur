//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_error_detailed_union_part() {
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

function f(a: XYZ)
    local b: { w: number } = a
end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        concat!(
            "Expected this to be '{ w: number }', but got 'X | Y | Z'; \n",
            "this is because \n\t",
            " * the 1st component of the union is `X`, which is not a subtype of `{ w: number }`\n\t",
            " * the 2nd component of the union is `Y`, which is not a subtype of `{ w: number }`\n\t",
            " * the 3rd component of the union is `Z`, which is not a subtype of `{ w: number }`"
        )
    } else {
        r#"Expected this to be '{ w: number }', but got 'X | Y | Z'
caused by:
  Not all union options are compatible.
Table type 'X' not compatible with type '{ w: number }' because the former is missing field 'w'"#
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
