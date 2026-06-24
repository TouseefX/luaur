//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1090:normalize_truthy_table_property_and_optional_table_with_optional_prop`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - calls -> method NormalizeFixture::typeFromNormal (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_truthy_table_property_and_optional_table_with_optional_prop

#[cfg(test)]
#[test]
fn normalize_truthy_table_property_and_optional_table_with_optional_prop() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_level::TypeLevel;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_analysis::type_aliases::props_type::Props;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = NormalizeFixture::default();

    let (truthy_type, optional_number_type, nil_type) = {
        let builtins = fixture.base.get_builtins();
        (
            builtins.truthyType,
            builtins.optionalNumberType,
            builtins.nilType,
        )
    };

    let mut t1_props = Props::new();
    t1_props.insert(String::from("x"), Property::rw_type_id(truthy_type));
    let t1 = fixture.arena.add_type(
        TableType::table_type_props_optional_table_indexer_type_level_table_state(
            &t1_props,
            None,
            TypeLevel::default(),
            TableState::Sealed,
        ),
    );

    let mut table_props = Props::new();
    table_props.insert(
        String::from("x"),
        Property::rw_type_id(optional_number_type),
    );
    let optional_table = fixture.arena.add_type(
        TableType::table_type_props_optional_table_indexer_type_level_table_state(
            &table_props,
            None,
            TypeLevel::default(),
            TableState::Sealed,
        ),
    );
    let t2 = fixture.arena.add_type(UnionType {
        options: alloc::vec![optional_table, nil_type],
    });

    let intersection = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![t2, t1],
    });

    let norm = fixture
        .normalize(intersection)
        .expect("expected normalized type");
    let ty = fixture.type_from_normal(norm.as_ref());

    assert_eq!("{ x: number }", to_string_type_id(ty));
}
