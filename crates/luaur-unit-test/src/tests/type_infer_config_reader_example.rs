//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2301:type_infer_config_reader_example`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> record Config (Config/include/Luau/Config.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_config_reader_example

#[cfg(test)]
#[test]
fn type_infer_config_reader_example() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/ConfigReader"),
        String::from(
            r#"
        --!strict
        local ConfigReader = {}
        ConfigReader.Defaults = {}

        local Defaults = ConfigReader.Defaults
        local Config = ConfigReader.Defaults

        function ConfigReader:read(config_name: string)
            if Config[config_name] ~= nil then
                return Config[config_name]
            elseif Defaults[config_name] ~= nil then
                return Defaults[config_name]
            else
                error(config_name .. " must be defined in Config")
            end
        end


        function ConfigReader:getFullConfigWithDefaults()
            local config = {}
            for key, val in pairs(ConfigReader.Defaults) do
                config[key] = val
            end
            for key, val in pairs(Config) do
                config[key] = val
            end
            return config
        end

        return ConfigReader
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/Util"),
        String::from(
            r#"
        --!strict
        local ConfigReader = require(script.Parent.ConfigReader)
        local _ = ConfigReader:read("foobar")()
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Util"), None);
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
