//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1532:type_function_getmetatable_returns_correct_metatable`
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
//!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_function_getmetatable_returns_correct_metatable

#[cfg(test)]
#[test]
fn type_function_getmetatable_returns_correct_metatable() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
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
        local metatable = { __index = { w = 4 } }
        local obj = setmetatable({x = 1, y = 2, z = 3}, metatable)
        type Metatable = getmetatable<typeof(obj)>
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "{ __index: { w: number } }",
        to_string_type_id_to_string_options_mut(
            fixture.base.require_type_alias(&String::from("Metatable")),
            ToStringOptions::to_string_options(true)
        )
    );
}
