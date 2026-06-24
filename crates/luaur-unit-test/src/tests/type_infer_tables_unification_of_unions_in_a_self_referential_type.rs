//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1200:type_infer_tables_unification_of_unions_in_a_self_referential_type`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_tables_unification_of_unions_in_a_self_referential_type

#[cfg(test)]
#[test]
fn type_infer_tables_unification_of_unions_in_a_self_referential_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::metatable_type::MetatableType;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = {}
        type AMT = { __mul: (A, A | number) -> A }
        local a: A
        local amt: AMT
        setmetatable(a, amt)

        type B = {}
        type BMT = { __mul: (B, A | B | number) -> A }
        local b: B
        local bmt: BMT
        setmetatable(b, bmt)

        a = b
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let amtv = unsafe {
        get_type_id::<MetatableType>(fixture.base.require_type_string(&String::from("a"))).as_ref()
    }
    .expect("expected MetatableType for a");
    assert_eq!(unsafe { follow_type_id(amtv.metatable()) }, unsafe {
        follow_type_id(fixture.base.require_type_string(&String::from("amt")))
    });

    let bmtv = unsafe {
        get_type_id::<MetatableType>(fixture.base.require_type_string(&String::from("b"))).as_ref()
    }
    .expect("expected MetatableType for b");
    assert_eq!(unsafe { follow_type_id(bmtv.metatable()) }, unsafe {
        follow_type_id(fixture.base.require_type_string(&String::from("bmt")))
    });
}
