//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:102:type_infer_oop_methods_are_topologically_sorted`
//! Source: `tests/TypeInfer.oop.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.oop.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.oop.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_oop_methods_are_topologically_sorted

#[cfg(test)]
#[test]
fn type_infer_oop_methods_are_topologically_sorted() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::primitive_type::PrimitiveType;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local T = {}

        function T:foo()
            return T:bar(999), T:bar("hi")
        end

        function T:bar(i)
            return i
        end

        local a, b = T:foo()
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let a_type = fixture.require_type_string(&String::from("a"));
    let b_type = fixture.require_type_string(&String::from("b"));
    assert_eq!(
        Some(PrimitiveType::Number),
        fixture.get_primitive_type(a_type)
    );
    assert_eq!(
        Some(PrimitiveType::String),
        fixture.get_primitive_type(b_type)
    );
}
