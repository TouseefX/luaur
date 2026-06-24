//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_non_recursive_aliases_that_reuse_a_generic_name() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Array<T> = { [number]: T }
        type Tuple<T, V> = Array<T | V>

        local p: Tuple<number, string>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{number | string}",
        to_string_type_id_to_string_options(
            fixture.require_type_string(&String::from("p")),
            &mut opts
        )
    );
}
