//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_table_annotation() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::primitive_type::PrimitiveType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x: {a: number, b: string} = {a=2, b="three"}
        local y = x.a
        local z = x.b
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let y_type = fixture.require_type_string(&String::from("y"));
    assert_eq!(
        Some(PrimitiveType::Number),
        fixture.get_primitive_type(y_type)
    );
    let z_type = fixture.require_type_string(&String::from("z"));
    assert_eq!(
        Some(PrimitiveType::String),
        fixture.get_primitive_type(z_type)
    );
}
