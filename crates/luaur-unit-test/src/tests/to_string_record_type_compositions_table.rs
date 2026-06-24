//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_record_type_compositions_table() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_detailed_to_string::to_string_detailed_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let check_result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Table = {}
    "#,
        ),
        None,
    );
    assert_eq!(0, check_result.errors.len(), "{:?}", check_result.errors);

    let mut opts = ToStringOptions::default();
    let ty = fixture.require_type_alias(&String::from("Table"));
    let result = to_string_detailed_type_id_to_string_options(ty, &mut opts);

    assert_eq!(1, result.type_spans.len());
    let span = result.type_spans[0];
    assert_eq!(0, span.start_pos());
    assert_eq!(5, span.end_pos());
    assert_eq!(ty, span.r#type());
}
