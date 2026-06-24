//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1790:type_function_user_udtf_generic_api_3`
//! Source: `tests/TypeFunction.user.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.user.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.user.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item type_function_user_udtf_generic_api_3

#[cfg(test)]
#[test]
fn type_function_user_udtf_generic_api_3() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type function pass()
    local T = types.generic("T")
    assert(T.tag == "generic")
    assert(T:name() == "T")
    assert(T:ispack() == false)

    local Us, Vs = types.generic("U", true), types.generic("V", true)
    assert(Us.tag == "generic")
    assert(Us:name() == "U")
    assert(Us:ispack() == true)

    local f = types.newfunction()
    f:setparameters({T}, Us);
    f:setreturns({T}, Vs);
    f:setgenerics({T, Us, Vs});
    return f
end

local function ok(idx: pass<>): <T, U..., V...>(T, U...) -> (T, V...) return idx end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
