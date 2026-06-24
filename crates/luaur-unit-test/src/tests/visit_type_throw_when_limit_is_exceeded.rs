//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:18:visit_type_throw_when_limit_is_exceeded`
//! Source: `tests/VisitType.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/VisitType.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
//! - incoming:
//!   - declares <- source_file tests/VisitType.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record RecursionLimitException (Analysis/include/Luau/RecursionCounter.h)
//!   - translates_to -> rust_item visit_type_throw_when_limit_is_exceeded

#[cfg(test)]
#[test]
fn visit_type_throw_when_limit_is_exceeded() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        let result = fixture.check_string_optional_frontend_options(
            &String::from(
                r#"
            local t : {a: {b: {c: {d: {e: boolean}}}}}
        "#,
            ),
            None,
        );
        assert!(result.errors.is_empty());

        let _sfi = ScopedFastInt::new(&luaur_common::FInt::LuauVisitRecursionLimit, 3);
        let t_type = fixture.require_type_string(&String::from("t"));

        assert!(std::panic::catch_unwind(|| to_string_type_id(t_type)).is_err());
    } else {
        let _sfi = ScopedFastInt::new(&luaur_common::FInt::LuauVisitRecursionLimit, 3);

        let result = fixture.check_string_optional_frontend_options(
            &String::from(
                r#"
            local t : {a: {b: {c: {d: {e: boolean}}}}}
        "#,
            ),
            None,
        );
        assert!(result.errors.is_empty());

        let t_type = fixture.require_type_string(&String::from("t"));

        assert!(std::panic::catch_unwind(|| to_string_type_id(t_type)).is_err());
    }
}
