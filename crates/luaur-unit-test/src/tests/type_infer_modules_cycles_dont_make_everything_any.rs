//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_cycles_dont_make_everything_any() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        --!strict
        local module = {}

        function module.foo()
            return 2
        end

        function module.bar()
            local m = require(game.B)
            return m.foo() + 1
        end

        return module
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        --!strict
        local module = {}

        function module.foo()
            return 2
        end

        function module.bar()
            local m = require(game.A)
            return m.foo() + 1
        end

        return module
    "#,
        ),
    );

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);

    let b_module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));
    assert_eq!("module", to_string_type_pack_id(b_module.return_type));
}
