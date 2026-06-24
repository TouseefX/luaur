//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:677:frontend_reports_errors_from_multiple_sources`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - translates_to -> rust_item frontend_reports_errors_from_multiple_sources

#[cfg(test)]
#[test]
fn frontend_reports_errors_from_multiple_sources() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/A"),
        String::from(
            r#"
        local a: number = 'oh no a type error'
        return {a=a}
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/B"),
        String::from(
            r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        local b: number = 'another one!  This is quite distressing!'
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!("game/Gui/Modules/A", result.errors[0].module_name);
    assert_eq!("game/Gui/Modules/B", result.errors[1].module_name);
}
