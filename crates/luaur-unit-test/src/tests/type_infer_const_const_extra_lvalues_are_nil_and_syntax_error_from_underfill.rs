//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:91:type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_underfill`
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
//!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_underfill

#[cfg(test)]
#[test]
fn type_infer_const_const_extra_lvalues_are_nil_and_syntax_error_from_underfill() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::syntax_error::SyntaxError;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _const2 = ScopedFastFlag::new(&FFlag::LuauConst2, true);
    let _underfill = ScopedFastFlag::new(&FFlag::LuauConstJustReportErrorForUnderfill, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        const X, Y, Z = 42, 13

        return { X, Y, Z }
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = unsafe { get_type_error::<SyntaxError>(&result.errors[0]).as_ref() }
        .expect("expected SyntaxError");
    assert_eq!("Missing initializer in const declaration", err.message());
}
