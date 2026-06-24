//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:68:type_function_basic_type_function`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_function_basic_type_function

#[cfg(test)]
#[test]
fn type_function_basic_type_function() {
    use crate::records::type_function_fixture::TypeFunctionFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = TypeFunctionFixture::type_function_fixture();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = Swap<number>
        type B = Swap<string>
        type C = Swap<boolean>

        local x = 123
        local y: Swap<typeof(x)> = "foo"
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_alias(&String::from("A")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_alias(&String::from("B")))
    );
    assert_eq!(
        "Swap<boolean>",
        to_string_type_id(fixture.base.require_type_alias(&String::from("C")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("y")))
    );
    assert_eq!(
        "Type function instance Swap<boolean> is uninhabited",
        to_string_type_error(&result.errors[0])
    );
}
