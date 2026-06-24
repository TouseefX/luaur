//! Ported from `tests/TypeInfer.operators.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:365:type_infer_operators_produce_the_correct_error_message_when_comparing_a_table_with_a_metatable_with_one_that_does_not`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_operators_produce_the_correct_error_message_when_comparing_a_table_with_a_metatable_with_one_that_does_not

#[cfg(test)]
#[test]
fn type_infer_operators_produce_the_correct_error_message_when_comparing_a_table_with_a_metatable_with_one_that_does_not(
) {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::records::generic_error::GenericError;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local M = {}
        function M.new()
            return setmetatable({}, M)
        end
        function M.__lt(left, right) return true end
        type M = typeof(M.new())

        local a = M.new()
        local b = {}
        local c = a < b -- line 10
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err =
        type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
    assert_eq!(
        "Types M and b cannot be compared with < because they do not have the same metatable",
        err.message()
    );
}
