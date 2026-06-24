//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_parameter_x_was_constrained_by_two_types_2() {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x): number?
            local y: string? = nil  -- 'y <: string?
            y = x                   -- 'y ~ 'x
            return y                -- 'y <: number?

                                    -- We therefore infer 'y <: (string | nil) & (number | nil)
                                    -- or 'y <: nil
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "(nil) -> number?",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
    );
}
