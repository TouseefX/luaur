//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1573:type_function_getmetatable_returns_correct_metatable_for_string`
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
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
//!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_function_getmetatable_returns_correct_metatable_for_string

#[cfg(test)]
#[test]
fn type_function_getmetatable_returns_correct_metatable_for_string() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_metatable_type::get_metatable_type_id_not_null_builtin_types;
    use luaur_analysis::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Metatable = getmetatable<string>
        type Metatable2 = getmetatable<"foo">
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let builtins = unsafe { &*fixture.base.builtin_types };
    let expected = get_metatable_type_id_not_null_builtin_types(builtins.string_type(), builtins)
        .expect("expected string metatable");
    let expected =
        to_string_type_id_to_string_options_mut(expected, ToStringOptions::to_string_options(true));

    assert_eq!(
        expected,
        to_string_type_id_to_string_options_mut(
            fixture.base.require_type_alias(&String::from("Metatable")),
            ToStringOptions::to_string_options(true)
        )
    );
    assert_eq!(
        expected,
        to_string_type_id_to_string_options_mut(
            fixture.base.require_type_alias(&String::from("Metatable2")),
            ToStringOptions::to_string_options(true)
        )
    );
}
