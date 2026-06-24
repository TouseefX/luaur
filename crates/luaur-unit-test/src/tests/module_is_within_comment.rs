//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:20:module_is_within_comment`
//! Source: `tests/Module.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Module.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Clone.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Module.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Module.test.cpp
//! - outgoing:
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::space (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item module_is_within_comment

#[cfg(test)]
#[test]
fn module_is_within_comment() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::is_within_comment_module_alt_b::is_within_comment_source_module_position;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        r#"
        --!strict
        local foo = {}
        function foo:bar() end

        --[[
            foo:
        ]] foo:bar()

        --[[]]--[[]] -- Two distinct comments that have zero characters of space between them.
    "#,
    );

    fixture.check_string_optional_frontend_options(&source, None);

    let source_module = fixture.get_main_source_module();
    let source_module = unsafe { &*source_module };

    assert_eq!(5, source_module.comment_locations.len());

    assert!(is_within_comment_source_module_position(
        source_module,
        Position::new(1, 15)
    ));
    assert!(is_within_comment_source_module_position(
        source_module,
        Position::new(6, 16)
    ));
    assert!(is_within_comment_source_module_position(
        source_module,
        Position::new(9, 13)
    ));
    assert!(is_within_comment_source_module_position(
        source_module,
        Position::new(9, 14)
    ));

    assert!(!is_within_comment_source_module_position(
        source_module,
        Position::new(2, 15)
    ));
    assert!(!is_within_comment_source_module_position(
        source_module,
        Position::new(7, 10)
    ));
    assert!(!is_within_comment_source_module_position(
        source_module,
        Position::new(7, 11)
    ));
}
