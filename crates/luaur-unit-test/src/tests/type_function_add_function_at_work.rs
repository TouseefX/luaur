//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:196:type_function_add_function_at_work`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_function_add_function_at_work

#[cfg(test)]
#[test]
fn type_function_add_function_at_work() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function add(a, b)
            return a + b
        end

        local a = add(1, 2)
        local b = add(1, "foo")
        local c = add("foo", 1)
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "add<number, string>",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "add<string, number>",
        to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
    assert_eq!(
        "Operator '+' could not be applied to operands of types number and string; there is no corresponding overload for __add",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "Operator '+' could not be applied to operands of types string and number; there is no corresponding overload for __add",
        to_string_type_error(&result.errors[1])
    );
}
