//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_do_not_modify_imported_types_2() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
export type Type = { x: { a: number } }
return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
local types = require(game.A)
type Type = types.Type
local x: Type = { x = { a = 2 } }
type Rename = typeof(x.x)
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
