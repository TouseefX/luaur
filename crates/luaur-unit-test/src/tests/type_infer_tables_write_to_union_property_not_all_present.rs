//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4899:type_infer_tables_write_to_union_property_not_all_present`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function fail (Config/src/Config.cpp)
//!   - type_ref -> record CannotAssignToNever (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> enum Reason (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_write_to_union_property_not_all_present

#[cfg(test)]
#[test]
fn type_infer_tables_write_to_union_property_not_all_present() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::reason::Reason;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::cannot_assign_to_never::CannotAssignToNever;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Animal = {tag: "Cat", meow: boolean} | {tag: "Dog", woof: boolean}
        function f(t: Animal)
            t.tag = "Dog"
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let tm = type_error_data_ref::<CannotAssignToNever>(&result.errors[0])
        .expect("expected CannotAssignToNever");
    assert_eq!(fixture.get_builtins().stringType, tm.rhsType());
    assert_eq!(Reason::PropertyNarrowed, tm.reason());
    assert_eq!(2, tm.cause().len());
    assert_eq!("\"Cat\"", to_string_type_id(tm.cause()[0]));
    assert_eq!("\"Dog\"", to_string_type_id(tm.cause()[1]));
}
