//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1402:type_infer_provisional_assert_and_many_nested_typeof_contexts`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_provisional_assert_and_many_nested_typeof_contexts

#[cfg(test)]
#[test]
fn type_infer_provisional_assert_and_many_nested_typeof_contexts() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local foo: unknown = nil :: any
        assert(typeof(foo) == "table")
        if typeof(typeof(foo.x)) == "string" then
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
