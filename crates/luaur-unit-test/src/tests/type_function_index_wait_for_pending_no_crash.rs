//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1004:type_function_index_wait_for_pending_no_crash`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_function_index_wait_for_pending_no_crash

#[cfg(test)]
#[test]
fn type_function_index_wait_for_pending_no_crash() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local PlayerData = {
            Coins = 0,
            Level = 1,
            Exp = 0,
            MaxExp = 100
        }
        type Keys = index<typeof(PlayerData), keyof<typeof(PlayerData)>>
        -- This function makes it think that there's going to be a pending expansion
        local function UpdateData(key: Keys, value)
            PlayerData[key] = value
        end
        UpdateData("Coins", 2)
    "#,
        ),
        None,
    );
}
