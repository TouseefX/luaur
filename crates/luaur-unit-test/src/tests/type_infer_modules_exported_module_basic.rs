//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_exported_module_basic() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _flags = [
        ScopedFastFlag::new(&FFlag::LuauConst2, true),
        ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
        ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
        ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        --!strict
        export local version = "1.0.0"
        export const name = "test module"
        export local count = 41

        count += 1
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        --!strict
        local A = require(game.A)

        local version = A.version
        local name = A.name
        local count = A.count
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

    let b = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));
    assert_eq!(
        "number",
        to_string_type_id(
            fixture
                .base
                .require_type_module_ptr_string(&b, &String::from("count"))
        )
    );
    assert_eq!(
        "string",
        to_string_type_id(
            fixture
                .base
                .require_type_module_ptr_string(&b, &String::from("version"))
        )
    );
    assert_eq!(
        "string",
        to_string_type_id(
            fixture
                .base
                .require_type_module_ptr_string(&b, &String::from("name"))
        )
    );
}
