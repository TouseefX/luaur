//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:96:visit_type_trace_a_simple_function`
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
//!   - type_ref -> record TracingVisitor (tests/VisitType.test.cpp)
//!   - translates_to -> rust_item visit_type_trace_a_simple_function

#[cfg(test)]
#[test]
fn visit_type_trace_a_simple_function() {
    use crate::records::fixture::Fixture;
    use crate::records::tracing_visitor::TracingVisitor;

    let mut fixture = Fixture::fixture_bool(false);
    let a = fixture.parse_type("(number, string) -> boolean");

    let mut vis = TracingVisitor::new(true, true);
    vis.run_type_id(a);

    assert_eq!(4, vis.trace.len());
    assert_eq!("(number, string) -> boolean", vis.trace[0]);
    assert_eq!("number", vis.trace[1]);
    assert_eq!("string", vis.trace[2]);
    assert_eq!("boolean", vis.trace[3]);
}
