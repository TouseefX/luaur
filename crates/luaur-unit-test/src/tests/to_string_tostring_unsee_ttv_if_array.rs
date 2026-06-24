//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_tostring_unsee_ttv_if_array() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x: {string}
        -- This code is constructed very specifically to use the same (by pointer
        -- identity) type in the function twice.
        local y: (typeof(x), typeof(x)) -> ()
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        "({string}, {string}) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
}
