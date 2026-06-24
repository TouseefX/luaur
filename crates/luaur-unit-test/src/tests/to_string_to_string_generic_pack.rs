//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_to_string_generic_pack() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
function foo(a, b) return a(b) end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "<a, b...>((a) -> (b...), a) -> (b...)",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
}
