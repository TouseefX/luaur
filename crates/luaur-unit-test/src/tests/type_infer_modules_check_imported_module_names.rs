//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_check_imported_module_names() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
return function(...) end
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
local l0 = require(game.A)
return l0
    "#,
        ),
    );

    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local l0 = require(game.B)
if true then
    local l1 = require(game.A)
end
return l0
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = fixture.base.get_main_module(false);
    assert!(!module.is_null());

    let scopes = unsafe { &(*module).scopes };
    assert_eq!(4, scopes.len());

    let root_scope = &scopes[0].1;
    let block_scope = &scopes[3].1;
    assert_eq!(
        Some(&String::from("game/B")),
        root_scope.imported_modules.get(&String::from("l0"))
    );
    assert_eq!(
        Some(&String::from("game/A")),
        block_scope.imported_modules.get(&String::from("l1"))
    );
}
