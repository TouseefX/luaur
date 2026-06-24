//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_local_that_will_be_assigned_later() {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x: string
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
