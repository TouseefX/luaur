//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:207:frontend_export_value_modules_have_typed_require_surface`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method AssemblyBuilderX64::inc (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item frontend_export_value_modules_have_typed_require_surface

#[cfg(test)]
#[test]
fn frontend_export_value_modules_have_typed_require_surface() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _flags = [
        ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
        ScopedFastFlag::new(&FFlag::LuauConst2, true),
        ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];
    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/ModuleA"),
        String::from(
            r#"
        --!strict
        export local version = "1.0.0"
        export const answer = 42

        export function inc(x: number): number
            return x + 1
        end
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("game/ModuleB"),
        String::from(
            r#"
        --!strict
        local M = require(game.ModuleA)

        local version: string = M.version
        local answer: number = M.answer
        local nextValue: number = M.inc(answer)

        return version, nextValue
    "#,
        ),
    );

    let a_result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/ModuleA"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/ModuleB"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let module_a = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/ModuleA"));

    let exports = first(module_a.return_type, true).expect("expected module return type");
    assert_eq!(
        "{ read answer: number, read inc: (number) -> number, read version: string }",
        to_string_type_id(exports)
    );
}
