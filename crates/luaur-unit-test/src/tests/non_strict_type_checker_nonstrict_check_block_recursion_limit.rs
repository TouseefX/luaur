//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_nonstrict_check_block_recursion_limit() {
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
    let code = "do ".repeat(limit) + "local a = 1" + &" end".repeat(limit);
    let code = String::from(code);
    let result = fixture.check_non_strict(&code);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
