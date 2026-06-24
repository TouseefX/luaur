//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:389:type_infer_check_type_infer_recursion_count`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function rep (tests/Fixture.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CodeTooComplex (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_check_type_infer_recursion_count

#[cfg(test)]
#[test]
fn type_infer_check_type_infer_recursion_count() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::records::code_too_complex::CodeTooComplex;
    use luaur_common::FInt;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let limit: usize = if cfg!(debug_assertions) { 350 } else { 600 };
    let _sfi = ScopedFastInt::new(&FInt::LuauCheckRecursionLimit, limit as i32);

    let mut fixture = Fixture::fixture_bool(false);
    let code = String::from("function f() return ")
        + &"{a=".repeat(limit)
        + "'a'"
        + &"}".repeat(limit)
        + " end";
    let result = fixture.check_string_optional_frontend_options(&code, None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<CodeTooComplex>(&result.errors[0]).expect("expected CodeTooComplex");
}
