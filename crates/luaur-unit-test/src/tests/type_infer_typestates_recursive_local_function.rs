//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_recursive_local_function() {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x)
            f(5)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
