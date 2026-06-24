//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:112:type_infer_const_const_syntax_error_in_annotation`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_const_const_syntax_error_in_annotation

#[cfg(test)]
#[test]
fn type_infer_const_const_syntax_error_in_annotation() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _const2 = ScopedFastFlag::new(&FFlag::LuauConst2, true);
    let _underfill = ScopedFastFlag::new(&FFlag::LuauConstJustReportErrorForUnderfill, true);
    let mut fixture = Fixture::fixture_bool(false);

    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        const foo: {
            bar
            baz
        } = {}

        return foo
    "#,
        ),
        None,
    );
}
