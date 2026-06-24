//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_read_only_properties() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = {x: string}
        type B = {read x: string}
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ x: string }",
        to_string_type_id_to_string_options(
            fixture.require_type_alias(&String::from("A")),
            &mut opts
        )
    );

    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ read x: string }",
        to_string_type_id_to_string_options(
            fixture.require_type_alias(&String::from("B")),
            &mut opts
        )
    );
}
