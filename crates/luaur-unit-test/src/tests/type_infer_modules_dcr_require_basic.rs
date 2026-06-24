//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_dcr_require_basic() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        --!strict
        return {
            a = 1,
        }
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        --!strict
        local A = require(game.A)

        local b = A.a
    "#,
        ),
    );

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
    let b_type = fixture
        .base
        .require_type_module_ptr_string(&b_module, &String::from("b"));
    assert_eq!("number", to_string_type_id(b_type));
}
