//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:278:type_infer_singletons_tagged_unions_immutable_tag`
//! Source: `tests/TypeInfer.singletons.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.singletons.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.singletons.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CannotAssignToNever (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> enum Reason (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_singletons_tagged_unions_immutable_tag

#[cfg(test)]
#[test]
fn type_infer_singletons_tagged_unions_immutable_tag() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::enums::reason::Reason;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::cannot_assign_to_never::CannotAssignToNever;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Dog = { tag: "Dog", howls: boolean }
        type Cat = { tag: "Cat", meows: boolean }
        type Animal = Dog | Cat
        local a: Animal = { tag = "Cat", meows = true }
        a.tag = "Dog"
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "expected errors");
    if !FFlag::DebugLuauForceOldSolver.get() {
        let tm = unsafe { get_type_error::<CannotAssignToNever>(&result.errors[0]).as_ref() }
            .expect("expected CannotAssignToNever");
        assert_eq!(fixture.get_builtins().stringType, tm.rhsType());
        assert_eq!(Reason::PropertyNarrowed, tm.reason());
        assert_eq!(2, tm.cause().len());
        assert_eq!(r#""Dog""#, to_string_type_id(tm.cause()[0]));
        assert_eq!(r#""Cat""#, to_string_type_id(tm.cause()[1]));
    }
}
