//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_long_disjunct_of_nil_is_nil_not_question_mark() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
      type nil_ty = nil | nil | nil | nil | nil
      local a : nil_ty = nil
  "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::default();
    opts.use_line_breaks = false;
    let a = fixture.require_type_string(&String::from("a"));
    assert_eq!("nil", to_string_type_id_to_string_options(a, &mut opts));
}
