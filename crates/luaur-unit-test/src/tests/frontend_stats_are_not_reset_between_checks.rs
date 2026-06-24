//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1049:frontend_stats_are_not_reset_between_checks`
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
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Stats (Analysis/include/Luau/Frontend.h)
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - translates_to -> rust_item frontend_stats_are_not_reset_between_checks

#[cfg(test)]
#[test]
fn frontend_stats_are_not_reset_between_checks() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
        --!strict
        local B = require(script.Parent.B)
        local foo = B.foo + 1
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
        --!strict
        return {foo = 1}
    "#,
        ),
    );

    let r1 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    assert_eq!(0, r1.errors.len(), "{:?}", r1.errors);

    let stats1 = fixture.get_frontend().stats;
    assert_eq!(2, stats1.files);

    fixture
        .get_frontend()
        .mark_dirty(&String::from("Module/A"), None);
    fixture
        .get_frontend()
        .mark_dirty(&String::from("Module/B"), None);

    let r2 = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    assert_eq!(0, r2.errors.len(), "{:?}", r2.errors);
    let stats2 = fixture.get_frontend().stats;

    assert_eq!(4, stats2.files);
}
