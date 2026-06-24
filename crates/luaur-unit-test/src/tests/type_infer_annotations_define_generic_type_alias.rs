//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_define_generic_type_alias() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Array<T> = {[number]: T}
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let main_module = fixture.get_main_module(false);
    assert!(!main_module.is_null(), "expected main module");
    let main_module = unsafe { &*main_module };
    assert!(main_module.has_module_scope());

    let scope = main_module.get_module_scope();
    let tf = scope
        .private_type_bindings
        .get("Array")
        .expect("expected Array private type binding");
    assert_eq!(1, tf.type_params().len());
}
