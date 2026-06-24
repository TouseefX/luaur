//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1809:type_infer_generics_generic_type_subtyping_nested_bounds_with_new_mappings`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_generics_generic_type_subtyping_nested_bounds_with_new_mappings

#[cfg(test)]
#[test]
fn type_infer_generics_generic_type_subtyping_nested_bounds_with_new_mappings() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Dispatch<A> = (A) -> ()
type BasicStateAction<S> = ((S) -> S) | S

function updateReducer<S, I, A>(reducer: (S, A) -> S, initialArg: I, init: ((I) -> S)?): (S, Dispatch<A>)
    return 1 :: any, 2 :: any
end

function basicStateReducer<S>(state: S, action: BasicStateAction<S>): S
    return action
end

function updateState<S>(initialState: (() -> S) | S): (S, Dispatch<BasicStateAction<S>>)
    return updateReducer(basicStateReducer, initialState)
end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
