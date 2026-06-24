//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_to_string_named_function_overrides_param_names() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use alloc::vec;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_named_function_to_string_alt_b::to_string_named_function_string_function_type_to_string_options;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function test(a, b : string, ... : number) return a end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("test"));
    let ftv = unsafe { get_type_id::<FunctionType>(follow_type_id(ty)).as_ref() }
        .expect("expected test to be a function type");
    let mut opts = ToStringOptions::default();
    opts.named_function_override_arg_names = vec![
        String::from("first"),
        String::from("second"),
        String::from("third"),
    ];
    assert_eq!(
        "test<a>(first: a, second: string, ...: number): a",
        to_string_named_function_string_function_type_to_string_options("test", ftv, &mut opts)
    );
}
