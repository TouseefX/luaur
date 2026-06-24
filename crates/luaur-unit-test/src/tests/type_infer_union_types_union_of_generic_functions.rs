//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_union_of_generic_functions() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x : <a>(a) -> a?)
            local y : (<a>(a?) -> a?) | (<b>(b) -> b) = x -- Not OK
        end
     "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty());
}
