//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1353:frontend_checked_modules_have_the_correct_mode`
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
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
//!   - translates_to -> rust_item frontend_checked_modules_have_the_correct_mode

#[cfg(test)]
#[test]
fn frontend_checked_modules_have_the_correct_mode() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_ast::enums::mode::Mode;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        --!nocheck
        local a: number = "five"
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        --!nonstrict
        local a = math.abs("five")
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("game/C"),
        String::from(
            r#"
        --!strict
        local a = 10
    "#,
        ),
    );

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/C"), None);

    let module_a = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/A"));
    assert_eq!(Mode::NoCheck, module_a.mode);

    let module_b = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));
    assert_eq!(Mode::Nonstrict, module_b.mode);

    let module_c = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/C"));
    assert_eq!(Mode::Strict, module_c.mode);
}
