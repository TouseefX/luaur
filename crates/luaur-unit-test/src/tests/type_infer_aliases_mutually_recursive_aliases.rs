//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_mutually_recursive_aliases() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        type T = { f: number, g: U }
        type U = { h: number, i: T? }
        local x: T = { f = 37, g = { h = 5, i = nil } }
        x.g.i = x
        local y: T = { f = 3, g = { h = 5, i = nil } }
        y.g.i = y
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
