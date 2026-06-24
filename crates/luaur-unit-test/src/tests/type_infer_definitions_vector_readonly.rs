//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:630:type_infer_definitions_vector_readonly`
//! Source: `tests/TypeInfer.definitions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> type_alias vec (Common/include/Luau/InsertionOrderedMap.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_definitions_vector_readonly

#[cfg(test)]
#[test]
fn type_infer_definitions_vector_readonly() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _lhs = ScopedFastFlag::new(&FFlag::LuauLValueCompoundAssignmentVisitLhs, true);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare extern type vector with
            read x: number
        end
    "#,
        ),
        false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local function read(n: number | boolean)
end

local function foo(vec: vector)
    read(vec.x)
    read(vec.x > 42)
    vec.x = 15
    vec.x -= 15
end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
        result.errors[0].data,
        TypeErrorData::PropertyAccessViolation(_)
    ));
    assert!(matches!(
        result.errors[1].data,
        TypeErrorData::PropertyAccessViolation(_)
    ));
    assert_eq!(8, result.errors[0].location.begin.line);
    assert_eq!(9, result.errors[1].location.begin.line);
}
