//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_function_type_with_argument_names_generic() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from("local function f<a...>(n: number, ...: a...): (a...) return ... end"),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::default();
    opts.function_type_arguments = true;
    assert_eq!(
        "<a...>(n: number, a...) -> (a...)",
        to_string_type_id_to_string_options(
            fixture.require_type_string(&String::from("f")),
            &mut opts
        )
    );
}
