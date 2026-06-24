#[cfg(test)]
#[test]
fn type_infer_functions_io_manager_oop_ish() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type IIOManager = {
            __index: IIOManager,
            write: (self: IOManager, text: string, label: string?) -> number,
        }

        export type IOManager = setmetatable<{
            buffer: {string},
            memory: { [string]: number }
        }, IIOManager>;

        local IO = {} :: IIOManager
        IO.__index = IO

        function IO:write(text, label)
            local _ = self
            local _ = text
            local _ = label
            return 42
        end

        return IO
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "IOManager",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 15,
            column: 25
        }))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 16,
            column: 25
        }))
    );
    assert_eq!(
        "string?",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 17,
            column: 25
        }))
    );
}
