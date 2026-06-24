//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_narrow_intersection_nevers() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
        &String::from(
            r#"
        declare class Player
            Character: unknown
        end
    "#,
        ),
        false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function foo(player: Player?)
            if player and player.Character then
                print(player.Character)
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Player & { read Character: ~(false?) }",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 3,
            column: 23,
        }))
    );
}
