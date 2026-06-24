//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1530:type_infer_provisional_oss_2305_keyof_index_example`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_provisional_oss_2305_keyof_index_example

#[cfg(test)]
#[test]
fn type_infer_provisional_oss_2305_keyof_index_example() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::uninhabited_type_function::UninhabitedTypeFunction;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _emplace = ScopedFastFlag::new(&FFlag::LuauRemoveConstraintSolverEmplace, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local settingsTable = {}

        type Settings = typeof(settingsTable)

        local settings = {}

        function settings.getTopic<T>(topic: keyof<Settings> & T): { setting: <U>(setting: keyof<index<Settings, T>> & U) -> (index<index<Settings, T>, U>) }
            return {
                setting = function<U>(setting: keyof<index<Settings, T>> & U): index<index<Settings, T>, U>
                    return settingsTable[topic][setting]
                end
            }
        end

        return settings
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[0]).unwrap_or_else(|| {
        panic!(
            "expected UninhabitedTypeFunction, got {:?}",
            result.errors[0]
        )
    });
}
