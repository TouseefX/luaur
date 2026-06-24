//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_module_type_conflict() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
export type T = { x: number }
return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
export type T = { x: string }
return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/C"),
        String::from(
            r#"
local A = require(game.A)
local B = require(game.B)
local a: A.T = { x = 2 }
local b: B.T = a
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/C"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "Expected this to be 'T' from 'game/B', but got 'T' from 'game/A'; \naccessing `x` results in `number` in the latter type and `string` in the former type, and `number` is not exactly `string`"
    } else {
        "Expected this to be exactly 'T' from 'game/B', but got 'T' from 'game/A'\ncaused by:\n  Property 'x' is not compatible.\nExpected this to be exactly 'string', but got 'number'"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
