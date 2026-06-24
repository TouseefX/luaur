//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:151:type_infer_const_const_recursive_function_works`
//! Source: `tests/TypeInfer.const.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.const.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.const.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_const_const_recursive_function_works

#[cfg(test)]
#[test]
fn type_infer_const_const_recursive_function_works() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _const2 = ScopedFastFlag::new(&FFlag::LuauConst2, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        const function f(x)
            f(5)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "(unknown) -> ()",
            to_string_type_id(fixture.require_type_string(&String::from("f")))
        );
    } else {
        assert_eq!(
            "(number) -> ()",
            to_string_type_id(fixture.require_type_string(&String::from("f")))
        );
    }
}
