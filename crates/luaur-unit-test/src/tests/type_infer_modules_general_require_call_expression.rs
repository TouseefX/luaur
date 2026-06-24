//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_general_require_call_expression() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
--!strict
return { def = 4 }
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
--!strict
local tbl = { abc = require(game.A) }
local a : string = ""
a = tbl.abc.def
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Expected this to be 'string', but got 'number'",
        to_string_type_error(&result.errors[0])
    );
}
