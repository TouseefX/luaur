//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_spooky_blocked_type_laundered_by_bound_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        local Cache = {}

        Cache.settings = {}

        Cache.data = {}

        function Cache.should_cache(url)
            url = url:split("?")[1]

            for key, _ in pairs(Cache.settings) do
                if url:match('') then
                    return key
                end
            end

            return ""
        end

        function Cache.is_cached(url, req_id)
            -- check local server cache first

            local setting_key = Cache.should_cache(url)
            local settings = Cache.settings[setting_key]

            if not setting_key then
                return false
            end

            if Cache.data[req_id] ~= nil then
                return true
            end

            if Cache.settings[setting_key].cache_globally then
                return false
            else
                return true
            end
        end

        function Cache.get_expire(url)
            local setting_key = Cache.should_cache(url)
            return Cache.settings[setting_key].expires or math.huge
        end

        return Cache
    "#,
        ),
    );

    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local _ = require(game.A);
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
