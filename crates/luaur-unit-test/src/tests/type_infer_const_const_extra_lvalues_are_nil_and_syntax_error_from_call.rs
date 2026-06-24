//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:66:type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_call`
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
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_call

#[cfg(test)]
#[test]
fn type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_call() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::count_mismatch::CountMismatch;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _const2 = ScopedFastFlag::new(&FFlag::LuauConst2, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function getparams(): (number, number)
            return 42, 13
        end

        const X, Y, Z = getparams()
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = unsafe { get_type_error::<CountMismatch>(&result.errors[0]).as_ref() }
        .expect("expected CountMismatch");
    assert_eq!(3, err.actual());
    assert_eq!(2, err.expected());
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("X")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("Y")))
    );
    assert_eq!(
        "nil",
        to_string_type_id(fixture.require_type_string(&String::from("Z")))
    );
}
