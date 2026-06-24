//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:113:type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable`
//! Source: `tests/TypeInfer.unknownnever.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable

#[cfg(test)]
#[test]
fn type_infer_unknownnever_type_packs_containing_never_is_itself_uninhabitable() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f() return "foo", 5 :: never end

        local x, y, z = f()
    "#,
        ),
        None,
    );

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "Function only returns 2 values, but 3 are required here",
            to_string_type_error(&result.errors[0])
        );

        assert_eq!(
            "string",
            to_string_type_id(fixture.require_type_string(&String::from("x")))
        );
        assert_eq!(
            "never",
            to_string_type_id(fixture.require_type_string(&String::from("y")))
        );
        assert_eq!(
            "nil",
            to_string_type_id(fixture.require_type_string(&String::from("z")))
        );
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);

        assert_eq!(
            "never",
            to_string_type_id(fixture.require_type_string(&String::from("x")))
        );
        assert_eq!(
            "never",
            to_string_type_id(fixture.require_type_string(&String::from("y")))
        );
        assert_eq!(
            "never",
            to_string_type_id(fixture.require_type_string(&String::from("z")))
        );
    }
}
