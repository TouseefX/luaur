//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1123:frontend_imported_table_modification_2`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item frontend_imported_table_modification_2

#[cfg(test)]
#[test]
fn frontend_imported_table_modification_2() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.get_frontend().options.retain_full_type_graphs = false;

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
--!nonstrict
local a = {}
a.x = 1
return a;
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
--!nonstrict
local a = require(script.Parent.A)
local b = {}
function a:b() end -- this should error, since A doesn't define a:b()
return b
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/C"),
        String::from(
            r#"
--!nonstrict
local a = require(script.Parent.A)
local b = require(script.Parent.B)
a:b() -- this should error, since A doesn't define a:b()
    "#,
        ),
    );

    let result_a = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    assert_eq!(0, result_a.errors.len(), "{:?}", result_a.errors);

    let result_b = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/B"), None);
    assert!(!result_b.errors.is_empty(), "expected errors");

    let result_c = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/C"), None);
    assert!(!result_c.errors.is_empty(), "expected errors");
}
