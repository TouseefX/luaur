//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:981:type_function_index_type_function_works`
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
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_function_index_type_function_works

#[cfg(test)]
#[test]
fn type_function_index_type_function_works() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
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
        type MyObject = {a: string, b: number, c: boolean}
        type IdxAType = index<MyObject, "a">
        type IdxBType = index<MyObject, keyof<MyObject>>

        local function ok(idx: IdxAType): string return idx end
        local function ok2(idx: IdxBType): string | number | boolean return idx end
        local function err(idx: IdxAType): boolean return idx end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    match &result.errors[0].data {
        TypeErrorData::TypeMismatch(tm) => {
            assert_eq!("boolean", to_string_type_id(tm.wanted_type));
            assert_eq!("string", to_string_type_id(tm.given_type));
        }
        other => panic!("expected TypeMismatch, got {other:?}"),
    }
}
