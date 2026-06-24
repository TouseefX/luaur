//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_record_type_compositions_union_handle_resorted_results() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_detailed_to_string::to_string_detailed_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let check_result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Zebra = {}
        type Alpha = {}

        type Composite = Zebra | Alpha
    "#,
        ),
        None,
    );
    assert_eq!(0, check_result.errors.len(), "{:?}", check_result.errors);

    let mut opts = ToStringOptions::default();
    let ty = fixture.require_type_alias(&String::from("Composite"));
    let result = to_string_detailed_type_id_to_string_options(ty, &mut opts);

    assert_eq!("Alpha | Zebra", result.name);
    assert_eq!(2, result.type_spans.len());

    let span_alpha = result.type_spans[0];
    assert_eq!(0, span_alpha.start_pos());
    assert_eq!(5, span_alpha.end_pos());
    assert_eq!(
        fixture.require_type_alias(&String::from("Alpha")),
        span_alpha.r#type()
    );

    let span_zebra = result.type_spans[1];
    assert_eq!(8, span_zebra.start_pos());
    assert_eq!(13, span_zebra.end_pos());
    assert_eq!(
        fixture.require_type_alias(&String::from("Zebra")),
        span_zebra.r#type()
    );
}
