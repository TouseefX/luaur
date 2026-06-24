//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1478:type_infer_generics_infer_generic_function_function_overloaded_pt_2`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_generics_infer_generic_function_function_overloaded_pt_2

#[cfg(test)]
#[test]
fn type_infer_generics_infer_generic_function_function_overloaded_pt_2() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local g12: (<T>(T, (T) -> T) -> T) & (<T>(T, T, (T, T) -> T) -> T)

        local a = g12({x=1}, function(x) return {x=-x.x} end)
        local b = g12({x=1}, {x=2}, function(x, y) return {x=x.x + y.x} end)
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "{ x: number } | { x: unm<unknown> }",
            to_string_type_id(fixture.require_type_string(&String::from("a")))
        );
        assert_eq!(
            "{ x: add<unknown, unknown> } | { x: number } | { x: number }",
            to_string_type_id(fixture.require_type_string(&String::from("b")))
        );
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "{| x: number |}",
            to_string_type_id(fixture.require_type_string(&String::from("a")))
        );
        assert_eq!(
            "{| x: number |}",
            to_string_type_id(fixture.require_type_string(&String::from("b")))
        );
    }
}
