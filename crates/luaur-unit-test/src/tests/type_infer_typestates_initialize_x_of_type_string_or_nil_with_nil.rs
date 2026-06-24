//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_initialize_x_of_type_string_or_nil_with_nil() {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x: string? = nil
        local a = x
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string?",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("a")))
    );
}
