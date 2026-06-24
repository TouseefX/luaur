//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:887:type_infer_provisional_expected_type_should_be_a_helpful_deduction_guide_for_function_calls`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_provisional_expected_type_should_be_a_helpful_deduction_guide_for_function_calls

#[cfg(test)]
#[test]
fn type_infer_provisional_expected_type_should_be_a_helpful_deduction_guide_for_function_calls() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Ref<T> = { val: T }

        local function useRef<T>(x: T): Ref<T?>
            return { val = x }
        end

        local x: Ref<number?> = useRef(nil)
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
}
