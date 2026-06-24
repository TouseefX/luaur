//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:719:type_function_keyof_oss_crash_gh_1161`
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
//!   - type_ref -> record FunctionExitsWithoutReturning (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_function_keyof_oss_crash_gh_1161

#[cfg(test)]
#[test]
fn type_function_keyof_oss_crash_gh_1161() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local EnumVariants = {
            ["a"] = 1, ["b"] = 2, ["c"] = 3
        }

        type EnumKey = keyof<typeof(EnumVariants)>

        function fnA<T>(i: T): keyof<T> end

        function fnB(i: EnumKey) end

        local result = fnA(EnumVariants)
        fnB(result)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
        matches!(
            &result.errors[0].data,
            TypeErrorData::FunctionExitsWithoutReturning(_)
        ),
        "{:?}",
        result.errors[0]
    );
}
