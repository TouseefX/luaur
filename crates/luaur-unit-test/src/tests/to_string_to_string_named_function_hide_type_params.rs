//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_to_string_named_function_hide_type_params() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_named_function_to_string_alt_b::to_string_named_function_string_function_type_to_string_options;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f<T>(x: T, g: <U>(T) -> U)): ()
        end
    "#,
        ),
        None,
    );

    let ty = fixture.require_type_string(&String::from("f"));
    let ftv = unsafe { get_type_id::<FunctionType>(follow_type_id(ty)).as_ref() }
        .expect("expected f to be a function type");
    let mut opts = ToStringOptions::default();
    opts.hide_named_function_type_parameters = true;
    assert_eq!(
        "f(x: T, g: <U>(T) -> U): ()",
        to_string_named_function_string_function_type_to_string_options("f", ftv, &mut opts)
    );
}
