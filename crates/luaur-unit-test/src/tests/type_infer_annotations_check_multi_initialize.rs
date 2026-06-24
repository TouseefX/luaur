//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_check_multi_initialize() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::type_mismatch::TypeMismatch;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: number, b: string = "one", 2
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }.is_some());
    assert!(unsafe { get_type_error::<TypeMismatch>(&result.errors[1]).as_ref() }.is_some());
}
