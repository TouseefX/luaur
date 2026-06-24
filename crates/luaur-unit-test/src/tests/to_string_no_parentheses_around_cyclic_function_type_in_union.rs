//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_no_parentheses_around_cyclic_function_type_in_union() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type F = ((() -> number)?) -> F?
        local function f(p) return f end
        local g: F = f
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        "t1 where t1 = ((() -> number)?) -> t1?",
        to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
}
