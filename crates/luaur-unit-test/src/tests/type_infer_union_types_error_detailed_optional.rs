//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_error_detailed_optional() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type X = { x: number }

local a: X? = { w = 4 }
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "Table type '{ w: number }' not compatible with type 'X' because the former is missing field 'x'"
    } else {
        r#"Expected this to be 'X?', but got 'a'
caused by:
  None of the union options are compatible. For example:
Table type 'a' not compatible with type 'X' because the former is missing field 'x'"#
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
