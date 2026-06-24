//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1656:type_function_error_suppression_should_work_on_type_functions`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method NormalizeFixture::normal (tests/Normalize.test.cpp)
//!   - calls -> method StringWriter::identifier (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_function_error_suppression_should_work_on_type_functions

#[cfg(test)]
#[test]
fn type_function_error_suppression_should_work_on_type_functions() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Colours = {
            Red = 1,
            Blue = 2,
            Green = 3,
            Taupe = 4,
        }

        -- namespace mixup here, Colours isn't a type, it's a normal identifier
        export type Colour = keyof<Colours>
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Unknown type 'Colours'",
        to_string_type_error(&result.errors[0])
    );
}
