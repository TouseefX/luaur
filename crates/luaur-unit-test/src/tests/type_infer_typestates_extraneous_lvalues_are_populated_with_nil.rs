//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_extraneous_lvalues_are_populated_with_nil() {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(): (string, number)
            return "hello", 5
        end

        local x, y, z = f()
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Function only returns 2 values, but 3 are required here",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("x")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("y")))
    );
    assert_eq!(
        "nil",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("z")))
    );
}
