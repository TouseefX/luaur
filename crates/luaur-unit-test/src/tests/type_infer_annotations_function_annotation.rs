//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_function_annotation() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local f: (number, string) -> number
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let f_type = fixture.require_type_string(&String::from("f"));
    let ftv = unsafe { get_type_id::<FunctionType>(follow_type_id(f_type)).as_ref() };
    assert!(ftv.is_some(), "expected function type");
}
