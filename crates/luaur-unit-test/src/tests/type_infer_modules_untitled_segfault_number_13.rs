//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_untitled_segfault_number_13() {
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
        -- minimized from roblox-requests/http/src/response.lua
        local Response = {}
        Response.__index = Response
        function Response.new(content_type)
            -- creates response object from original request and roblox http response
            local self = setmetatable({}, Response)
            self.content_type = content_type
            return self
        end

        function Response:xml(ignore_content_type)
            if ignore_content_type or self.content_type:find("+xml") or self.content_type:find("/xml") then
            else
            end
        end

        ---------------

        return Response
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
