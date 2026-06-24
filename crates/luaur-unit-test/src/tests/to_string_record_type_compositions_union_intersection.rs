//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_record_type_compositions_union_intersection() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_detailed_to_string::to_string_detailed_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let check_result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type TableA = {}
        type TableB = {}

        type Composite1 = TableA | TableB
        type Composite2 = TableA & TableB
    "#,
        ),
        None,
    );
    assert_eq!(0, check_result.errors.len(), "{:?}", check_result.errors);

    for alias_name in ["Composite1", "Composite2"] {
        let mut opts = ToStringOptions::default();
        let ty = fixture.require_type_alias(&String::from(alias_name));
        let result = to_string_detailed_type_id_to_string_options(ty, &mut opts);

        assert_eq!(2, result.type_spans.len(), "{}", alias_name);

        let span_a = result.type_spans[0];
        assert_eq!(0, span_a.start_pos(), "{}", alias_name);
        assert_eq!(6, span_a.end_pos(), "{}", alias_name);
        assert_eq!(
            fixture.require_type_alias(&String::from("TableA")),
            span_a.r#type(),
            "{}",
            alias_name
        );

        let span_b = result.type_spans[1];
        assert_eq!(9, span_b.start_pos(), "{}", alias_name);
        assert_eq!(15, span_b.end_pos(), "{}", alias_name);
        assert_eq!(
            fixture.require_type_alias(&String::from("TableB")),
            span_b.r#type(),
            "{}",
            alias_name
        );
    }
}
