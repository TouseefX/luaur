//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1420:frontend_no_separate_caches_with_the_new_solver`
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
//!   - type_ref -> record FrontendOptions (Analysis/include/Luau/Frontend.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
//!   - translates_to -> rust_item frontend_no_separate_caches_with_the_new_solver

#[cfg(test)]
#[test]
fn frontend_no_separate_caches_with_the_new_solver() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::frontend_options::FrontendOptions;
    use luaur_ast::enums::mode::Mode;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        --!nonstrict
        local exports = {}
        function exports.hello() end
        return exports
    "#,
        ),
    );

    let mut opts = FrontendOptions::default();
    opts.for_autocomplete = true;

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), Some(opts));

    assert!(!fixture
        .get_frontend()
        .module_resolver_for_autocomplete
        .modules
        .contains_key(&String::from("game/A")));

    let module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/A"));
    assert_eq!(Mode::Nonstrict, module.mode);
}
