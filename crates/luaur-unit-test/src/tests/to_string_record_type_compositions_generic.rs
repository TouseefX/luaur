//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_record_type_compositions_generic() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_detailed_to_string::to_string_detailed_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let check_result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Object = {}
        type Box<T> = { inner: T }

        local x: Box<Object>
    "#,
        ),
        None,
    );
    assert_eq!(0, check_result.errors.len(), "{:?}", check_result.errors);

    let mut opts = ToStringOptions::default();
    let ty = fixture.require_type_string(&String::from("x"));
    let result = to_string_detailed_type_id_to_string_options(ty, &mut opts);

    assert_eq!(2, result.type_spans.len());

    let span_box = result.type_spans[0];
    assert_eq!(0, span_box.start_pos());
    assert_eq!(3, span_box.end_pos());
    assert_eq!(ty, span_box.r#type());

    let span_object = result.type_spans[1];
    assert_eq!(4, span_object.start_pos());
    assert_eq!(10, span_object.end_pos());
    assert_eq!(
        fixture.require_type_alias(&String::from("Object")),
        span_object.r#type()
    );
}
