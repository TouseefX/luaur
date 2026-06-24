//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:634:frontend_recheck_if_dependent_script_has_a_parse_error`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - translates_to -> rust_item frontend_recheck_if_dependent_script_has_a_parse_error

#[cfg(test)]
#[test]
fn frontend_recheck_if_dependent_script_has_a_parse_error() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture
        .base
        .base
        .file_resolver
        .source
        .insert(String::from("Modules/A"), String::from("local"));
    fixture.base.base.file_resolver.source.insert(
        String::from("Modules/B"),
        String::from(
            r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        return {}
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!("Modules/A", result.errors[0].module_name);

    let result2 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);
    assert_eq!(1, result2.errors.len(), "{:?}", result2.errors);
    assert_eq!(result.errors[0], result2.errors[0]);
}
