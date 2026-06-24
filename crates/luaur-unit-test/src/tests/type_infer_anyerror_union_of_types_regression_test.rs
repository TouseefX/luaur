//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.anyerror.test.cpp:393:type_infer_anyerror_union_of_types_regression_test`
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
//!   - calls -> macro tonumber (VM/src/lvm.h)
//!   - translates_to -> rust_item type_infer_anyerror_union_of_types_regression_test

#[cfg(test)]
#[test]
fn type_infer_anyerror_union_of_types_regression_test() {
    use crate::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local stat
stat = stat and tonumber(stat) or stat
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
