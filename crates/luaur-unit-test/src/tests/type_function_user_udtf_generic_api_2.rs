//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1767:type_function_user_udtf_generic_api_2`
//! Source: `tests/TypeFunction.user.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.user.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.user.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item type_function_user_udtf_generic_api_2

#[cfg(test)]
#[test]
fn type_function_user_udtf_generic_api_2() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type function pass(arg)
    local generics = arg:generics()
    local T = generics[1]
    local f = types.newfunction()
    f:setparameters({T, T});
    f:setreturns({T});
    f:setgenerics({T});
    return f
end

type test = <T, U>(T, { x: <T>(y: T) -> (), y: U }, U) -> ()

local function ok(idx: pass<test>): <T>(T, T) -> (T) return idx end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
