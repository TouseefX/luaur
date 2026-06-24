//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1065:type_function_index_type_function_works_w_generic_types`
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
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_function_index_type_function_works_w_generic_types

#[cfg(test)]
#[test]
fn type_function_index_type_function_works_w_generic_types() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function access<T, K>(tbl: T & {}, key: K): index<T, K>
            return tbl[key]
        end

        local subjects = {
            english = "boring",
            math = "fun"
        }

        local key: "english" = "english"
        local a: string = access(subjects, key)
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
