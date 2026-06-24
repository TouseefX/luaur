//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:463:type_infer_singletons_parametric_tagged_union_alias`
//! Source: `tests/TypeInfer.singletons.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.singletons.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.singletons.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_singletons_parametric_tagged_union_alias

#[cfg(test)]
#[test]
fn type_infer_singletons_parametric_tagged_union_alias() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Ok<T> = {success: true, result: T}
        type Err<T> = {success: false, error: T}
        type Result<O, E> = Ok<O> | Err<E>

        local a : Result<string, number> = {success = false, result = "hotdogs"}
        -- local b : Result<string, number> = {success = true, result = "hotdogs"}
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Table type '{ result: string, success: false }' not compatible with type 'Err<number>' because the former is missing field 'error'",
        to_string_type_error(&result.errors[0])
    );
}
