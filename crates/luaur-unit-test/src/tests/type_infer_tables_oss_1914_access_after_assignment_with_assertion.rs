#[cfg(test)]
#[test]
fn type_infer_tables_oss_1914_access_after_assignment_with_assertion() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        type WallHolder = {
            __type: "Model",
            Wall: {
                __type: "BasePart",
                age: number,
            },
        }

        local walls = {
            { name = "Part1" },
            { name = "Part2" },
            { name = "Wall" },
        }

        local baseWall: WallHolder?
        for _, wall in walls do
            if wall.name == "Wall" then
                baseWall = wall :: WallHolder
            end
        end
        assert(baseWall, "Failed to get base wall when creating room props")

        local myAge = baseWall.Wall.age
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("myAge")))
    );
}
