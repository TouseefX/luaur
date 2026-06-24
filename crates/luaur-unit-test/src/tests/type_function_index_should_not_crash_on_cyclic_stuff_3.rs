//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:954:type_function_index_should_not_crash_on_cyclic_stuff_3`
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
//!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_function_index_should_not_crash_on_cyclic_stuff_3

#[cfg(test)]
#[test]
fn type_function_index_should_not_crash_on_cyclic_stuff_3() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local PlayerData = {
            Coins = 0,
            Level = 1,
            Exp = 0,
            MapExp = 100,
        }

        type Keys = index<typeof(PlayerData), true>

        local function UpdateData(key: Keys, value)
            PlayerData[key] = value
        end

        UpdateData("Coins", 2)
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "unknown",
        to_string_type_id(fixture.base.require_type_alias(&String::from("Keys")))
    );
}
