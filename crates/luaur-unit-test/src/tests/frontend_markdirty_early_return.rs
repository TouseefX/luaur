//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1306:frontend_markdirty_early_return`
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
//!   - translates_to -> rust_item frontend_markdirty_early_return

#[cfg(test)]
#[test]
fn frontend_markdirty_early_return() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use alloc::vec::Vec;

    let module_name = String::from("game/Gui/Modules/A");
    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        module_name.clone(),
        String::from(
            r#"
        return 1
    "#,
        ),
    );

    {
        let mut marked_dirty: Vec<String> = Vec::new();
        fixture
            .get_frontend()
            .mark_dirty(&module_name, Some(&mut marked_dirty));
        assert!(marked_dirty.is_empty(), "{:?}", marked_dirty);
    }

    fixture.get_frontend().parse_module_name(&module_name);

    {
        let mut marked_dirty: Vec<String> = Vec::new();
        fixture
            .get_frontend()
            .mark_dirty(&module_name, Some(&mut marked_dirty));
        assert!(!marked_dirty.is_empty());
    }
}
