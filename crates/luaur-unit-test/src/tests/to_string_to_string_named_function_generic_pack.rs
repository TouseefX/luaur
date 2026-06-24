//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_to_string_named_function_generic_pack() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_named_function_to_string::to_string_named_function_string_function_type;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(a: number, b: string) end
        local function test<T..., U...>(...: T...): U...
            f(...)
            return 1, 2, 3
        end
    "#,
        ),
        None,
    );

    let ty = fixture.require_type_string(&String::from("test"));
    let ftv = unsafe { get_type_id::<FunctionType>(follow_type_id(ty)).as_ref() }
        .expect("expected test to be a function type");
    assert_eq!(
        "test<T..., U...>(...: T...): U...",
        to_string_named_function_string_function_type("test", ftv)
    );
}
