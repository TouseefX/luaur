//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1778:type_infer_generics_generic_implicit_explicit_name_clash`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - translates_to -> rust_item type_infer_generics_generic_implicit_explicit_name_clash

#[cfg(test)]
#[test]
fn type_infer_generics_generic_implicit_explicit_name_clash() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function apply<a>(func, argument: a)
            return func(argument)
        end
    "#,
        ),
        None,
    );

    assert_eq!(
        "<a, b...>((a) -> (b...), a) -> (b...)",
        to_string_type_id(fixture.require_type_string(&String::from("apply")))
    );
}
