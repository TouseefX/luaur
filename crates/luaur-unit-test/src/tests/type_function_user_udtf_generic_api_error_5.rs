//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2025:type_function_user_udtf_generic_api_error_5`
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
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item type_function_user_udtf_generic_api_error_5

#[cfg(test)]
#[test]
fn type_function_user_udtf_generic_api_error_5() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type function get()
    local T, Ts = types.generic("T"), types.generic("T", true)
    return types.newfunction({ head = {T} }, { tail = Ts }, {T, Ts})
end
local function ok(idx: get<>): false return idx end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Duplicate type parameter 'T'",
        to_string_type_error(&result.errors[0])
    );
}
