//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:180:visit_type_can_be_configured_not_to_skip_bound_types`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TracingVisitor (tests/VisitType.test.cpp)
//!   - translates_to -> rust_item visit_type_can_be_configured_not_to_skip_bound_types

#[cfg(test)]
#[test]
fn visit_type_can_be_configured_not_to_skip_bound_types() {
    use crate::records::fixture::Fixture;
    use crate::records::tracing_visitor::TracingVisitor;
    use luaur_analysis::type_aliases::bound_type::BoundType;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let number_type = fixture.get_builtins().numberType;

    let a = fixture.arena.add_type(BoundType::bound_t(number_type));

    let mut vis = TracingVisitor::new(true, false);
    vis.run_type_id(a);

    assert_eq!(2, vis.trace.len());
    assert_eq!("number", vis.trace[0]);
    assert_eq!("number", vis.trace[1]);
}
