//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.anyerror.test.cpp:366:type_infer_anyerror_type_error_addition`
//! Source: `tests/TypeInfer.anyerror.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.anyerror.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.anyerror.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_anyerror_type_error_addition

#[cfg(test)]
#[test]
fn type_infer_anyerror_type_error_addition() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local foo = makesandwich()
local bar = foo.nutrition + 100
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Unknown global 'makesandwich'; consider assigning to it first",
        to_string_type_error(&result.errors[0])
    );
}
