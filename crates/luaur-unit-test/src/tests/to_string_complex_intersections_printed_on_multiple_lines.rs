//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_complex_intersections_printed_on_multiple_lines() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: string & number & boolean
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::default();
    opts.use_line_breaks = true;
    opts.composite_types_single_line_limit = 2;
    let a = fixture.require_type_string(&String::from("a"));
    assert_eq!(
        "boolean\n& number\n& string",
        to_string_type_id_to_string_options(a, &mut opts)
    );
}
