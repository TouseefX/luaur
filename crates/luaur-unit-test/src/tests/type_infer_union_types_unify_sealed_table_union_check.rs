//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_unify_sealed_table_union_check() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
 -- the difference between this and unify_unsealed_table_union_check is the type annotation on x
local t = { x = 3, y = true }
local x: { x: number } = t
type A = number?
type B = string?
local y: { x: number, y: A | B }
-- Shouldn't typecheck!
y = x
-- If it does, we can convert any type to any other type
y.y = 5
local oh : boolean = t.y
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty());
}
