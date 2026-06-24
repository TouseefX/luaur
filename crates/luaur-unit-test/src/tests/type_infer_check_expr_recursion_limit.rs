//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:442:type_infer_check_expr_recursion_limit`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function rep (tests/Fixture.cpp)
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CodeTooComplex (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_check_expr_recursion_limit

#[cfg(test)]
#[test]
fn type_infer_check_expr_recursion_limit() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::code_too_complex::CodeTooComplex;
    use luaur_common::{DFInt, FInt};

    let limit: usize = if cfg!(debug_assertions) { 250 } else { 500 };

    let _luau_recursion_limit = ScopedFastInt::new(&FInt::LuauRecursionLimit, limit as i32 * 2);
    let _luau_check_recursion_limit =
        ScopedFastInt::new(&FInt::LuauCheckRecursionLimit, limit as i32 - 100);
    let _luau_constraint_generator_recursion_limit = ScopedFastInt::new(
        &DFInt::LuauConstraintGeneratorRecursionLimit,
        limit as i32 - 100,
    );
    let _luau_subtyping_recursion_limit =
        ScopedFastInt::new(&DFInt::LuauSubtypingRecursionLimit, limit as i32 - 100);

    let mut fixture = Fixture::fixture_bool(false);
    let code = String::from(r#"("foo")"#) + &":lower()".repeat(limit);
    let result = fixture.check_string_optional_frontend_options(&code, None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
        type_error_data_ref::<CodeTooComplex>(&result.errors[0]).is_some(),
        "Expected CodeTooComplex but got {}",
        to_string_type_error(&result.errors[0])
    );
}
