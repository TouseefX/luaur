//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_require() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        local function hooty(x: number): string
            return "Hi there!"
        end

        return {hooty=hooty}
    "#,
        ),
    );

    let source_b = if !FFlag::DebugLuauForceOldSolver.get() {
        r#"
            local Hooty = require(game.A)

            local h = 4
            local i = Hooty.hooty(h)
        "#
    } else {
        r#"
            local Hooty = require(game.A)

            local h -- free!
            local i = Hooty.hooty(h)
        "#
    };

    fixture
        .base
        .file_resolver
        .source
        .insert(String::from("game/B"), String::from(source_b));

    let a_result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let b_module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));

    let i_type = fixture
        .base
        .require_type_module_ptr_string(&b_module, &String::from("i"));
    assert_eq!("string", to_string_type_id(i_type));

    let h_type = fixture
        .base
        .require_type_module_ptr_string(&b_module, &String::from("h"));
    assert_eq!("number", to_string_type_id(h_type));
}
