//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:883:non_strict_type_checker_nonstrict_check_expr_recursion_limit`
//! Source: `tests/NonStrictTypeChecker.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrict (tests/NonStrictTypeChecker.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function rep (tests/Fixture.cpp)
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - translates_to -> rust_item non_strict_type_checker_nonstrict_check_expr_recursion_limit

#[cfg(test)]
#[test]
fn non_strict_type_checker_nonstrict_check_expr_recursion_limit() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_common::{DFInt, FFlag, FInt};

    let limit: usize = 250;

    let _sff = ScopedFastFlag::new(&FFlag::LuauAddRecursionCounterToNonStrictTypeChecker, true);

    let _luau_non_strict_type_checker_recursion_limit = ScopedFastInt::new(
        &FInt::LuauNonStrictTypeCheckerRecursionLimit,
        limit as i32 - 100,
    );
    let _luau_constraint_generator_recursion_limit = ScopedFastInt::new(
        &DFInt::LuauConstraintGeneratorRecursionLimit,
        limit as i32 + 500,
    );
    let _luau_check_recursion_limit =
        ScopedFastInt::new(&FInt::LuauCheckRecursionLimit, limit as i32 + 500);

    let mut fixture = NonStrictTypeCheckerFixture::default();
    let code = String::from(r#"("foo")"#) + &":lower()".repeat(limit);
    let result = fixture.check_non_strict(&code);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
