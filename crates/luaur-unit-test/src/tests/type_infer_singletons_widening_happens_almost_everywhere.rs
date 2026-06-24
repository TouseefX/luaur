//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:536:type_infer_singletons_widening_happens_almost_everywhere`
//! Source: `tests/TypeInfer.singletons.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.singletons.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.singletons.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_singletons_widening_happens_almost_everywhere

#[cfg(test)]
#[test]
fn type_infer_singletons_widening_happens_almost_everywhere() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local foo: "foo" = "foo"
        local copy = foo
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        r#""foo""#
    } else {
        "string"
    };
    assert_eq!(
        expected,
        to_string_type_id(fixture.require_type_string(&String::from("copy")))
    );
}
