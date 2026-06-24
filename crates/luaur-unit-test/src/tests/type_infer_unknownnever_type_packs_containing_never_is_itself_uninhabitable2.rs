//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:140:type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable2`
//! Source: `tests/TypeInfer.unknownnever.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable2

#[cfg(test)]
#[test]
fn type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable2() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(): (string, never) return "", 5 :: never end
        local function g(): (never, string) return 5 :: never, "" end

        local x1, x2 = f()
        local y1, y2 = g()
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "string",
            to_string_type_id(fixture.require_type_string(&String::from("x1")))
        );
        assert_eq!(
            "never",
            to_string_type_id(fixture.require_type_string(&String::from("x2")))
        );
        assert_eq!(
            "never",
            to_string_type_id(fixture.require_type_string(&String::from("y1")))
        );
        assert_eq!(
            "string",
            to_string_type_id(fixture.require_type_string(&String::from("y2")))
        );
    } else {
        assert_eq!(
            "never",
            to_string_type_id(fixture.require_type_string(&String::from("x1")))
        );
        assert_eq!(
            "never",
            to_string_type_id(fixture.require_type_string(&String::from("x2")))
        );
        assert_eq!(
            "never",
            to_string_type_id(fixture.require_type_string(&String::from("y1")))
        );
        assert_eq!(
            "never",
            to_string_type_id(fixture.require_type_string(&String::from("y2")))
        );
    }
}
