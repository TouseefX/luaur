//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_module_type_conflict_instantiated() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
export type Wrap<T> = { x: T }
return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
local A = require(game.A)
export type T = A.Wrap<number>
return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/C"),
        String::from(
            r#"
local A = require(game.A)
export type T = A.Wrap<string>
return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/D"),
        String::from(
            r#"
local A = require(game.B)
local B = require(game.C)
local a: A.T = { x = 2 }
local b: B.T = a
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/D"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "Expected this to be 'T' from 'game/C', but got 'T' from 'game/B'; \naccessing `x` results in `number` in the latter type and `string` in the former type, and `number` is not exactly `string`"
    } else {
        "Expected this to be exactly 'T' from 'game/C', but got 'T' from 'game/B'\ncaused by:\n  Property 'x' is not compatible.\nExpected this to be exactly 'string', but got 'number'"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
