//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:804:type_function_didnt_quite_exceed_distributivity_limits`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_function_didnt_quite_exceed_distributivity_limits

#[cfg(test)]
#[test]
fn type_function_didnt_quite_exceed_distributivity_limits() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_common::{DFInt, FFlag};

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let _limit = ScopedFastInt::new(&DFInt::LuauTypeFamilyApplicationCartesianProductLimit, 20);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
        &String::from(
            r#"
        declare class A
            function __mul(self, rhs: unknown): A
        end

        declare class B
            function __mul(self, rhs: unknown): B
        end

        declare class C
            function __mul(self, rhs: unknown): C
        end

        declare class D
            function __mul(self, rhs: unknown): D
        end
    "#,
        ),
        false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = mul<A | B | C | D, A | B | C | D>
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
