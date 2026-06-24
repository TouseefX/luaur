//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:50:type_infer_tc_propagation`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_tc_propagation

#[cfg(test)]
#[test]
fn type_infer_tc_propagation() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::primitive_type::PrimitiveType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture
        .check_string_optional_frontend_options(&String::from("local a = 7   local b = a"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let b_type = fixture.require_type_string(&String::from("b"));
    assert_eq!(
        Some(PrimitiveType::Number),
        fixture.get_primitive_type(b_type)
    );
}
