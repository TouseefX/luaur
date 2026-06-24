//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_require_failed_module() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
return unfortunately()
    "#,
        ),
    );

    let a_result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(!a_result.errors.is_empty(), "{:?}", a_result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local ModuleA = require(game.A)
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module_a = fixture.base.require_type_string(&String::from("ModuleA"));
    assert_eq!("*error-type*", to_string_type_id(module_a));
}
