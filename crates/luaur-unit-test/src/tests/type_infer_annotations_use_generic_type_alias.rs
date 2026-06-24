//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_use_generic_type_alias() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::type_mismatch::TypeMismatch;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Array<T> = {[number]: T}   -- 1
        local p: Array<number> = {}     -- 2
        p[1] = 5                        -- 3 OK
        p[2] = 'hello'                  -- 4 Error.
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(4, result.errors[0].location.begin.line);
    assert!(unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }.is_some());
}
