//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_optional_union_members() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a = { a = { x = 1, y = 2 }, b = 3 }
        type A = typeof(a)
        function f(b: A?)
            return b.a.y
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Value of type 'A?' could be nil",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "(A?) -> number",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
}
