//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:144:visit_type_detects_cycles`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - calls -> method Variant::emplace (Common/include/Luau/Variant.h)
//!   - type_ref -> record TracingVisitor (tests/VisitType.test.cpp)
//!   - translates_to -> rust_item visit_type_detects_cycles

#[cfg(test)]
#[test]
fn visit_type_detects_cycles() {
    use crate::records::fixture::Fixture;
    use crate::records::tracing_visitor::TracingVisitor;
    use alloc::collections::BTreeMap;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::as_mutable_type::as_mutable_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::blocked_type::BlockedType;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_level::TypeLevel;
    use luaur_analysis::type_aliases::type_variant::TypeVariant;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let (number_type, empty_type_pack) = {
        let builtins = fixture.get_builtins();
        (builtins.numberType, builtins.emptyTypePack)
    };

    let f_type = fixture.arena.add_type(BlockedType::default());

    let mut props = BTreeMap::new();
    props.insert(
        alloc::string::String::from("method"),
        Property::rw_type_id(f_type),
    );
    let t_type = fixture.arena.add_type(
        TableType::table_type_props_optional_table_indexer_type_level_table_state(
            &props,
            None,
            TypeLevel::default(),
            TableState::Sealed,
        ),
    );

    let args = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[t_type, number_type]);
    unsafe {
        (*as_mutable_type_id(f_type)).ty = TypeVariant::Function(FunctionType::function_type_new(
            args,
            empty_type_pack,
            None,
            false,
        ));
    }

    let mut vis = TracingVisitor::new(true, true);
    vis.run_type_id(f_type);

    assert_eq!(3, vis.trace.len());
    assert_eq!("t1 where t1 = ({ method: t1 }, number) -> ()", vis.trace[0]);
    assert_eq!("t1 where t1 = { method: (t1, number) -> () }", vis.trace[1]);
    assert_eq!("number", vis.trace[2]);

    assert_eq!(1, vis.cycles.len());
    assert_eq!(
        "t1 where t1 = ({ method: t1 }, number) -> ()",
        to_string_type_id(vis.cycles[0])
    );
}
