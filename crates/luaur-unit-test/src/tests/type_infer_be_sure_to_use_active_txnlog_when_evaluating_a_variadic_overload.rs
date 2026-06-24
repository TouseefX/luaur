//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1614:type_infer_be_sure_to_use_active_txnlog_when_evaluating_a_variadic_overload`
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
//!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
//!   - translates_to -> rust_item type_infer_be_sure_to_use_active_txnlog_when_evaluating_a_variadic_overload

#[cfg(test)]
#[test]
fn type_infer_be_sure_to_use_active_txnlog_when_evaluating_a_variadic_overload() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function concat<T>(target: {T}, ...: {T} | T): {T}
            return (nil :: any) :: {T}
        end

        local res = concat({"alic"}, 1, 2)
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    for error in &result.errors {
        assert_eq!(5, error.location.begin.line, "{:?}", result.errors);
    }
}
