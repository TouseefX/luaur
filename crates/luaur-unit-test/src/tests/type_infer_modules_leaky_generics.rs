//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_leaky_generics() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Cache = {}

        Cache.settings = {}

        function Cache.should_cache(url)
            for key, _ in pairs(Cache.settings) do
                return key
            end

            return ""
        end

        function Cache.is_cached(url)
            local setting_key = Cache.should_cache(url)
            local settings = Cache.settings[setting_key]

            return settings
        end

        return Cache
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "(unknown) -> unknown",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 13,
            column: 23
        }))
    );
}
