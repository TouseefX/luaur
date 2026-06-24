//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:914:type_infer_provisional_floating_generics_should_not_be_allowed`
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
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_provisional_floating_generics_should_not_be_allowed

#[cfg(test)]
#[test]
fn type_infer_provisional_floating_generics_should_not_be_allowed() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local assign : <T, U, V, W>(target: T, source0: U?, source1: V?, source2: W?, ...any) -> T & U & V & W = (nil :: any)

        -- We have a big problem here: The generics U, V, and W are not bound to anything!
        -- Things get strange because of this.
        local benchmark = assign({})
        local options = benchmark.options
        do
            local resolve2: any = nil
            options.fn({
                resolve = function(...)
                    resolve2(...)
                end,
            })
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
