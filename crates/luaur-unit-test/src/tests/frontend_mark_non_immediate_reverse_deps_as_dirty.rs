//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:609:frontend_mark_non_immediate_reverse_deps_as_dirty`
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
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - translates_to -> rust_item frontend_mark_non_immediate_reverse_deps_as_dirty

#[cfg(test)]
#[test]
fn frontend_mark_non_immediate_reverse_deps_as_dirty() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use alloc::vec::Vec;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/A"),
        String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/B"),
        String::from(
            r#"
        return require(game:GetService('Gui').Modules.A)
    "#,
        ),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/C"),
        String::from(
            r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {c_value = B.hello}
    "#,
        ),
    );

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/C"), None);

    let mut marked_dirty: Vec<String> = Vec::new();
    fixture
        .get_frontend()
        .mark_dirty(&String::from("game/Gui/Modules/A"), Some(&mut marked_dirty));

    assert_eq!(3, marked_dirty.len(), "{:?}", marked_dirty);
    assert!(marked_dirty.contains(&String::from("game/Gui/Modules/A")));
    assert!(marked_dirty.contains(&String::from("game/Gui/Modules/B")));
    assert!(marked_dirty.contains(&String::from("game/Gui/Modules/C")));
}
