//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:127:type_function_resolve_deep_functions`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_function_resolve_deep_functions

#[cfg(test)]
#[test]
fn type_function_resolve_deep_functions() {
    use crate::records::type_function_fixture::TypeFunctionFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = TypeFunctionFixture::type_function_fixture();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x: Swap<Swap<Swap<string>>>
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("x")))
    );
}
