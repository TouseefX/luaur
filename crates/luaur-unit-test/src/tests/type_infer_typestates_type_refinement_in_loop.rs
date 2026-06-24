//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_type_refinement_in_loop() {
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
        local function onEachString(t: { string | number })
            for _, v in t do
                if type(v) ~= "string" then
                    continue
                end
                print(v)
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "number | string",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 4,
            column: 24
        }))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 7,
            column: 22
        }))
    );
}
