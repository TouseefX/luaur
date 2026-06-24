//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:665:type_function_vector_2_multiply_is_overloaded`
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
//!   - translates_to -> rust_item type_function_vector_2_multiply_is_overloaded

#[cfg(test)]
#[test]
fn type_function_vector_2_multiply_is_overloaded() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local v = Vector2.New(1, 2)

        local v2 = v * 1.5
        local v3 = v * v
        local v4 = v * "Hello" -- line 5
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(5, result.errors[0].location.begin.line);
    assert_eq!(5, result.errors[0].location.end.line);

    assert_eq!(
        "Vector2",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("v2")))
    );
    assert_eq!(
        "Vector2",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("v3")))
    );
    assert_eq!(
        "mul<Vector2, string>",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("v4")))
    );
}
