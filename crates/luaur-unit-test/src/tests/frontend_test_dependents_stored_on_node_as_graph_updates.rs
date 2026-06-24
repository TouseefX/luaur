//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1658:frontend_test_dependents_stored_on_node_as_graph_updates`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - type_ref -> record DenseHashMap (Common/include/Luau/DenseHash.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record SourceNode (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item frontend_test_dependents_stored_on_node_as_graph_updates

#[cfg(test)]
#[test]
fn frontend_test_dependents_stored_on_node_as_graph_updates() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::{String, ToString};
    use alloc::vec::Vec;
    use std::collections::BTreeMap;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    let update_source = |fixture: &mut FrontendFixture, name: &str, source: &str| {
        fixture
            .base
            .base
            .file_resolver
            .source
            .insert(name.to_string(), source.to_string());
        fixture.get_frontend().mark_dirty(&name.to_string(), None);
    };

    let validate_matches_require_lists = |fixture: &mut FrontendFixture, message: &str| {
        let frontend = fixture.get_frontend();
        let mut dependents: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for (module_name, node) in &frontend.source_nodes {
            for dep in node.require_set.iter() {
                dependents
                    .entry(dep.clone())
                    .or_default()
                    .push(module_name.clone());
            }
        }

        for (module_name, node) in &frontend.source_nodes {
            if let Some(expected_dependents) = dependents.get(module_name) {
                for dep in expected_dependents {
                    assert!(
                        node.dependents.contains(dep),
                        "Mismatch in dependents for {module_name}: {message}"
                    );
                }
            }
        }
    };

    let validate_second_depends_on_first =
        |fixture: &mut FrontendFixture, from: &str, to: &str, expected: bool| {
            let frontend = fixture.get_frontend();
            let from_node = frontend
                .source_nodes
                .get(from)
                .unwrap_or_else(|| panic!("expected source node {from}"));
            assert_eq!(
                expected,
                from_node.dependents.contains(&to.to_string()),
                "Expected {from} to {}have a reverse dependency on {to}",
                if expected { "" } else { "not " }
            );
        };

    // C -> B -> A
    {
        update_source(
            &mut fixture,
            "game/Gui/Modules/A",
            "return {hello=5, world=true}",
        );
        update_source(
            &mut fixture,
            "game/Gui/Modules/B",
            r#"
            return require(game:GetService('Gui').Modules.A)
        "#,
        );
        update_source(
            &mut fixture,
            "game/Gui/Modules/C",
            r#"
            local Modules = game:GetService('Gui').Modules
            local B = require(Modules.B)
            return {c_value = B}
        "#,
        );
        fixture
            .get_frontend()
            .check_module_name_optional_frontend_options(&"game/Gui/Modules/C".to_string(), None);

        validate_matches_require_lists(&mut fixture, "Initial check");

        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/A",
            "game/Gui/Modules/B",
            true,
        );
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/B",
            "game/Gui/Modules/C",
            true,
        );
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/C",
            "game/Gui/Modules/A",
            false,
        );
    }

    // C -> B, A
    {
        update_source(
            &mut fixture,
            "game/Gui/Modules/B",
            r#"
            return 1
        "#,
        );
        fixture
            .get_frontend()
            .check_module_name_optional_frontend_options(&"game/Gui/Modules/C".to_string(), None);

        validate_matches_require_lists(&mut fixture, "Removing dependency B->A");
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/A",
            "game/Gui/Modules/B",
            false,
        );
    }

    // C -> B -> A
    {
        update_source(
            &mut fixture,
            "game/Gui/Modules/B",
            r#"
            return require(game:GetService('Gui').Modules.A)
        "#,
        );
        fixture
            .get_frontend()
            .check_module_name_optional_frontend_options(&"game/Gui/Modules/C".to_string(), None);

        validate_matches_require_lists(&mut fixture, "Adding back B->A");
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/A",
            "game/Gui/Modules/B",
            true,
        );
    }

    // C -> B -> A, D -> (C,B,A)
    {
        update_source(
            &mut fixture,
            "game/Gui/Modules/D",
            r#"
            local C = require(game:GetService('Gui').Modules.C)
            local B = require(game:GetService('Gui').Modules.B)
            local A = require(game:GetService('Gui').Modules.A)
            return {d_value = C.c_value}
        "#,
        );
        fixture
            .get_frontend()
            .check_module_name_optional_frontend_options(&"game/Gui/Modules/D".to_string(), None);

        validate_matches_require_lists(&mut fixture, "Adding D->C, D->B, D->A");
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/A",
            "game/Gui/Modules/D",
            true,
        );
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/B",
            "game/Gui/Modules/D",
            true,
        );
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/C",
            "game/Gui/Modules/D",
            true,
        );
    }

    // B -> A, C <-> D
    {
        update_source(
            &mut fixture,
            "game/Gui/Modules/D",
            "return require(game:GetService('Gui').Modules.C)",
        );
        update_source(
            &mut fixture,
            "game/Gui/Modules/C",
            "return require(game:GetService('Gui').Modules.D)",
        );
        fixture
            .get_frontend()
            .check_module_name_optional_frontend_options(&"game/Gui/Modules/D".to_string(), None);

        validate_matches_require_lists(&mut fixture, "Adding cycle D->C, C->D");
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/C",
            "game/Gui/Modules/D",
            true,
        );
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/D",
            "game/Gui/Modules/C",
            true,
        );
    }

    // B -> A, C -> D, D -> error
    {
        update_source(
            &mut fixture,
            "game/Gui/Modules/D",
            "return require(game:GetService('Gui').Modules.C.)",
        );
        fixture
            .get_frontend()
            .check_module_name_optional_frontend_options(&"game/Gui/Modules/D".to_string(), None);

        validate_matches_require_lists(&mut fixture, "Adding error dependency D->C.");
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/D",
            "game/Gui/Modules/C",
            true,
        );
        validate_second_depends_on_first(
            &mut fixture,
            "game/Gui/Modules/C",
            "game/Gui/Modules/D",
            false,
        );
    }
}
