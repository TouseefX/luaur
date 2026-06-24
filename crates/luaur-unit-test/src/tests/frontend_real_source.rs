//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:122:frontend_real_source`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//! - outgoing:
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - type_ref -> record NaiveFileResolver (tests/Frontend.test.cpp)
//!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
//!   - translates_to -> rust_item frontend_real_source

#[cfg(test)]
#[test]
fn frontend_real_source() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use crate::records::naive_file_resolver::NaiveFileResolver;
    use luaur_analysis::functions::trace_requires::trace_requires;
    use luaur_analysis::records::type_check_limits::TypeCheckLimits;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };
    let program = fixture.base.base.parse(
        r#"
        return function()
            local Modules = game:GetService("CoreGui").Gui.Modules

            local Roact = require(Modules.Common.Roact)
            local Rodux = require(Modules.Common.Rodux)

            local AppReducer = require(Modules.LuaApp.AppReducer)
            local AEAppReducer = require(Modules.LuaApp.Reducers.AEReducers.AEAppReducer)
            local AETabList = require(Modules.LuaApp.Components.Avatar.UI.Views.Portrait.AETabList)
            local mockServices = require(Modules.LuaApp.TestHelpers.mockServices)
            local DeviceOrientationMode = require(Modules.LuaApp.DeviceOrientationMode)
            local MockAvatarEditorTheme = require(Modules.LuaApp.TestHelpers.MockAvatarEditorTheming)
            local FFlagAvatarEditorEnableThemes = settings():GetFFlag("AvatarEditorEnableThemes2")
        end
    "#,
        &ParseOptions::default(),
    );

    let mut naive_file_resolver = NaiveFileResolver::default();

    let result = trace_requires(
        &mut naive_file_resolver.base.base,
        program,
        String::new(),
        &TypeCheckLimits::default(),
    );
    assert_eq!(8, result.require_list.len());
}
