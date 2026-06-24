//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:405:type_function_keyof_type_function_errors_if_it_has_nontable_part`
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
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item type_function_keyof_type_function_errors_if_it_has_nontable_part

#[cfg(test)]
#[test]
fn type_function_keyof_type_function_errors_if_it_has_nontable_part() {
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
        type MyObject = { x: number, y: number, z: number }
        type KeysOfMyObject = keyof<MyObject | boolean>

        local function err(idx: KeysOfMyObject): "x" | "y" | "z" return idx end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Type 'MyObject | boolean' does not have keys, so 'keyof<MyObject | boolean>' is invalid",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "Type 'MyObject | boolean' does not have keys, so 'keyof<MyObject | boolean>' is invalid",
        to_string_type_error(&result.errors[1])
    );
}
