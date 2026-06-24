//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:653:frontend_produce_errors_for_unchanged_file_with_a_syntax_error`
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
//!   - translates_to -> rust_item frontend_produce_errors_for_unchanged_file_with_a_syntax_error

#[cfg(test)]
#[test]
fn frontend_produce_errors_for_unchanged_file_with_a_syntax_error() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("Modules/A"),
        String::from("oh no a blatant syntax error!!"),
    );

    let one = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);
    let two = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);

    assert!(
        !one.errors.is_empty(),
        "expected first check to report errors"
    );
    assert!(
        !two.errors.is_empty(),
        "expected second check to report errors"
    );
}
