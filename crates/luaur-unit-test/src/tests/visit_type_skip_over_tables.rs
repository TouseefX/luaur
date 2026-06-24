//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:131:visit_type_skip_over_tables`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record TableSkippingVisitor (tests/VisitType.test.cpp)
//!   - translates_to -> rust_item visit_type_skip_over_tables

#[cfg(test)]
#[test]
fn visit_type_skip_over_tables() {
    use crate::records::fixture::Fixture;
    use crate::records::table_skipping_visitor::TableSkippingVisitor;

    let mut fixture = Fixture::fixture_bool(false);
    let a =
        fixture.parse_type("(number, string, {x: number, y: number}) -> {x: number, y: number}");

    let mut vis = TableSkippingVisitor::new();
    vis.run_type_id(a);

    assert_eq!(3, vis.trace.len());
    assert_eq!(
        "(number, string, { x: number, y: number }) -> { x: number, y: number }",
        vis.trace[0]
    );
    assert_eq!("number", vis.trace[1]);
    assert_eq!("string", vis.trace[2]);
}
