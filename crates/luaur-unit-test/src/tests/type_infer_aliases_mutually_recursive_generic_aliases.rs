//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_mutually_recursive_generic_aliases() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        type T<a> = { f: a, g: U<a> }
        type U<a> = { h: a, i: T<a>? }
        local x: T<number> = { f = 37, g = { h = 5, i = nil } }
        x.g.i = x
        local y: T<string> = { f = "hi", g = { h = "lo", i = nil } }
        y.g.i = y
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
