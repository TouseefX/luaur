//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_to_string_named_function_type_annotation_has_partial_argnames() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_named_function_to_string::to_string_named_function_string_function_type;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local f: (number, y: number) -> number
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("f"));
    let ftv = unsafe { get_type_id::<FunctionType>(follow_type_id(ty)).as_ref() }
        .expect("expected f to be a function type");
    assert_eq!(
        "f(_: number, y: number): number",
        to_string_named_function_string_function_type("f", ftv)
    );
}
