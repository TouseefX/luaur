//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1682:type_infer_builtins_table_dot_clone_type_states`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_builtins_table_dot_clone_type_states

#[cfg(test)]
#[test]
fn type_infer_builtins_table_dot_clone_type_states() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t1 = {}
        t1.x = 5
        local t2 = table.clone(t1)
        t2.y = 6
        t1.z = 3
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let expected_t1 = if !FFlag::DebugLuauForceOldSolver.get() {
        "{ x: number, z: number }"
    } else {
        "{| x: number, z: number |}"
    };
    let expected_t2 = if !FFlag::DebugLuauForceOldSolver.get() {
        "{ x: number, y: number }"
    } else {
        "{| x: number, y: number |}"
    };

    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        expected_t1,
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("t1")),
            &mut opts
        )
    );
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        expected_t2,
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("t2")),
            &mut opts
        )
    );
}
