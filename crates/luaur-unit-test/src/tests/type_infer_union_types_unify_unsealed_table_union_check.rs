//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_unify_unsealed_table_union_check() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local x = { x = 3 }
type A = number?
type B = string?
local y: { x: number, y: A | B }
y = x
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local x = { x = 3 }

local a: number? = 2
local y = {}
y.x = 2
y.y = a

y = x
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
