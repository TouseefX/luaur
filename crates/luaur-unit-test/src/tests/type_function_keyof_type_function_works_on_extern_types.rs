//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:585:type_function_keyof_type_function_works_on_extern_types`
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
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_function_keyof_type_function_works_on_extern_types

#[cfg(test)]
#[test]
fn type_function_keyof_type_function_works_on_extern_types() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type KeysOfMyObject = keyof<BaseClass>

        local function ok(idx: KeysOfMyObject): "BaseMethod" | "BaseField" | "Touched" return idx end
        local function err(idx: KeysOfMyObject): "BaseMethod" return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
        TypeErrorData::TypeMismatch(tm) => {
            assert_eq!("\"BaseMethod\"", to_string_type_id(tm.wanted_type));
            assert_eq!(
                "\"BaseField\" | \"BaseMethod\" | \"Touched\"",
                to_string_type_id(tm.given_type)
            );
        }
        other => panic!("expected TypeMismatch, got {other:?}"),
    }
}
