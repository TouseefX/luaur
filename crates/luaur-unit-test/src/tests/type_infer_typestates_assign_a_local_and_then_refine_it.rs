//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_assign_a_local_and_then_refine_it() {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x: string?)
            x = nil

            if typeof(x) == "string" then
                local y: typeof(x) = "hello"
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Expected this to be unreachable, but got 'string'",
        to_string_type_error(&result.errors[0])
    );
}
