//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1275:frontend_parse_only`
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
//!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item frontend_parse_only

#[cfg(test)]
#[test]
fn frontend_parse_only() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

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
        local b: number = 2
    "#,
        ),
    );

    fixture
        .get_frontend()
        .parse_module_name(&String::from("game/Gui/Modules/B"));

    assert!(fixture
        .get_frontend()
        .source_nodes
        .contains_key(&String::from("game/Gui/Modules/A")));
    assert!(fixture
        .get_frontend()
        .source_nodes
        .contains_key(&String::from("game/Gui/Modules/B")));

    let node = fixture
        .get_frontend()
        .source_nodes
        .get(&String::from("game/Gui/Modules/B"))
        .cloned()
        .expect("expected source node");
    assert!(node
        .require_set
        .contains(&String::from("game/Gui/Modules/A")));
    assert_eq!(1, node.require_locations.len());
    assert_eq!(
        Location::new(
            Position {
                line: 2,
                column: 18,
            },
            Position {
                line: 2,
                column: 36,
            },
        ),
        node.require_locations[0].1
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!("game/Gui/Modules/A", result.errors[0].module_name);
    assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
    );
}
