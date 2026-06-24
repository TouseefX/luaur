//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/RuntimeLimits.test.cpp:489:runtime_limits_test_generic_pruning_recursion_limit`
//! Source: `tests/RuntimeLimits.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/RuntimeLimits.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//! - incoming:
//!   - declares <- source_file tests/RuntimeLimits.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item runtime_limits_test_generic_pruning_recursion_limit

#[cfg(test)]
#[test]
fn runtime_limits_test_generic_pruning_recursion_limit() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::{FFlag, FInt};

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _steps = ScopedFastInt::new(&FInt::LuauGenericCounterMaxSteps, 1);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function get(scale)
            print(scale.Do.Re.Mi)
        end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "<a>({ read Do: { read Re: { read Mi: a } } }) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("get")))
    );
}
