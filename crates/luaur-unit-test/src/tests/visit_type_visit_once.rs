//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:192:visit_type_visit_once`
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
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TracingVisitor (tests/VisitType.test.cpp)
//!   - translates_to -> rust_item visit_type_visit_once

#[cfg(test)]
#[test]
fn visit_type_visit_once() {
    use crate::records::fixture::Fixture;
    use crate::records::tracing_visitor::TracingVisitor;
    use alloc::collections::BTreeMap;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_level::TypeLevel;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let number_type = fixture.get_builtins().numberType;

    let mut props = BTreeMap::new();
    props.insert(
        alloc::string::String::from("x"),
        Property::rw_type_id(number_type),
    );
    let x_table = fixture.arena.add_type(
        TableType::table_type_props_optional_table_indexer_type_level_table_state(
            &props,
            None,
            TypeLevel::default(),
            TableState::Sealed,
        ),
    );

    let arg_pack = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[x_table, x_table]);
    let ret_pack = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[x_table]);
    let fn_ty = fixture.arena.add_type(FunctionType::function_type_new(
        arg_pack, ret_pack, None, false,
    ));

    {
        let mut vis = TracingVisitor::new(true, true);
        vis.run_type_id(fn_ty);

        assert_eq!(3, vis.trace.len());
        assert_eq!(
            "({ x: number }, { x: number }) -> { x: number }",
            vis.trace[0]
        );
        assert_eq!("{ x: number }", vis.trace[1]);
        assert_eq!("number", vis.trace[2]);
        assert_eq!(0, vis.cycles.len());
    }

    {
        let mut vis = TracingVisitor::new(false, true);
        vis.run_type_id(fn_ty);

        assert_eq!(7, vis.trace.len());
        assert_eq!(
            "({ x: number }, { x: number }) -> { x: number }",
            vis.trace[0]
        );
        assert_eq!("{ x: number }", vis.trace[1]);
        assert_eq!("{ x: number }", vis.trace[2]);
        assert_eq!("{ x: number }", vis.trace[3]);
        assert_eq!("number", vis.trace[4]);
        assert_eq!("number", vis.trace[5]);
        assert_eq!("number", vis.trace[6]);
        assert_eq!(0, vis.cycles.len());
    }
}
