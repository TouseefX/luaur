//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:664:frontend_produce_errors_for_unchanged_file_with_errors`
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
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item frontend_produce_errors_for_unchanged_file_with_errors

#[cfg(test)]
#[test]
fn frontend_produce_errors_for_unchanged_file_with_errors() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("Modules/A"),
        String::from("local p: number = 'oh no a type error'"),
    );

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);

    fixture.base.base.file_resolver.source.insert(
        String::from("Modules/A"),
        String::from(
            "local p = 4 -- We have fixed the problem, but we didn't tell the getFrontend(). so it will not recheck this file!",
        ),
    );
    let second_result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);

    assert_eq!(1, second_result.errors.len(), "{:?}", second_result.errors);
}
