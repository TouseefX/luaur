//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_type_assertion_expr() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let _result =
        fixture.check_string_optional_frontend_options(&String::from("local a = 55 :: any"), None);

    assert_eq!(
        "any",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
}
