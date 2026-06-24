//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_saturate_to_first_type_pack() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T<A, B, C...> = { fn: (A, B) -> C... }
        local x: T<string, number, string, boolean>
        local f = x.fn
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "T<string, number, string, boolean>",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
        "(string, number) -> (string, boolean)",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
}
