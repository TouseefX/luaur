//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_cross_module_table_freeze() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use luaur_common::FFlag;

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
        return table.freeze(require(game.A))
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

    let a_module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/A"));
    assert_eq!(
        "{ a: number }",
        to_string_type_pack_id(a_module.return_type)
    );

    let b_module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "{ read a: number }"
    } else {
        "{ a: number }"
    };
    assert_eq!(expected, to_string_type_pack_id(b_module.return_type));
}
