//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:363:type_var_visit_once`
//! Source: `tests/TypeVar.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeVar.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeVar.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record VisitCountTracker (tests/TypeVar.test.cpp)
//!   - translates_to -> rust_item type_var_visit_once

#[cfg(test)]
#[test]
fn type_var_visit_once() {
    use crate::records::fixture::Fixture;
    use crate::records::visit_count_tracker::VisitCountTracker;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
        r#"
type T = { a: number, b: () -> () }
local b: (T, T, T) -> T
"#,
    );
    let result = fixture.check_string_optional_frontend_options(&source, None);
    assert!(
        result.errors.is_empty(),
        "expected no errors, got {:?}",
        result.errors
    );

    let b_type = fixture.require_type_string(&String::from("b"));

    let mut tester = VisitCountTracker::new();
    tester.traverse(b_type);

    for count in tester.ty_visits.values() {
        assert_eq!(1, *count);
    }

    for count in tester.tp_visits.values() {
        assert_eq!(1, *count);
    }
}
