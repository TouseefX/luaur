//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1076:normalize_cyclic_stack_overflow_2`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record NormalizedType (Analysis/include/Luau/Normalize.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_cyclic_stack_overflow_2

#[cfg(test)]
#[test]
fn normalize_cyclic_stack_overflow_2() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::type_aliases::props_type::Props;
    use luaur_common::FInt;

    let _sfi = ScopedFastInt::new(&FInt::LuauTypeInferRecursionLimit, 165);

    let mut fixture = NormalizeFixture::default();
    fixture
        .unifier_state
        .set_recursion_limit(FInt::LuauTypeInferRecursionLimit.get() as i32);

    let t1 = fixture.arena.add_type(TableType::table_type());
    let t2 = fixture.arena.add_type(TableType::table_type());
    let t3 = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![t1, t2],
    });

    unsafe {
        let t1_table = get_mutable_type_id::<TableType>(t1)
            .as_mut()
            .expect("expected t1 table type");
        let mut t1_props = Props::new();
        t1_props.insert(String::from("foo"), Property::readonly(t3));
        t1_table.props = t1_props;

        let t2_table = get_mutable_type_id::<TableType>(t2)
            .as_mut()
            .expect("expected t2 table type");
        let mut t2_props = Props::new();
        t2_props.insert(String::from("foo"), Property::readonly(t1));
        t2_table.props = t2_props;
    }

    let normalized = fixture.normalize(t3);
    assert!(normalized.is_some(), "expected normalized type");
}
