//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_fuzzer_normalized_type_variables_are_bad() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local _
        while _[""] do
            _, _ = nil
            while _.n0 do
                _, _ = nil
            end
            _, _ = nil
        end
        while _[""] do
            while if _ then if _ then _ else "" else "" do
                _, _ = nil
                do
                end
                _, _, _ = nil
            end
            _, _ = nil
            _, _, _ = nil
            while _.readi16 do
                _, _ = nil
            end
            _, _ = nil
        end
    "#,
        ),
        None,
    );

    let result = result;
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
