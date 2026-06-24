//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_type_alias_always_resolve_to_a_real_type() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = B
        type B = C
        type C = number

        local aa:A
    "#,
        ),
        None,
    );

    let f_type = fixture.require_type_string(&String::from("aa"));
    let number_type = fixture.get_builtins().numberType;
    assert_eq!(number_type, unsafe { follow_type_id(f_type) });
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
