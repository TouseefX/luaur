//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_throw_in_if_branch() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local x
        local coinflip : () -> boolean = (nil :: any)

        if coinflip () then
            error("You lose.")
        else
            x = "I win."
        end

        print(x)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 11,
            column: 14
        }))
    );
}
