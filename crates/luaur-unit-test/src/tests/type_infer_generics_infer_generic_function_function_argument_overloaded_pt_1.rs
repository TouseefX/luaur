//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1454:type_infer_generics_infer_generic_function_function_argument_overloaded_pt_1`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_generics_infer_generic_function_function_argument_overloaded_pt_1

#[cfg(test)]
#[test]
#[ignore = "leaked-generic divergence in new-solver generic inference: var `a` infers to \
`add<unknown, unknown> | number` instead of `number | number` (2 UninhabitedTypeFunction errors \
vs upstream's 1). Both call sites select the correct overload (verified) and store a resolved \
overload whose generic T = `add<t1,t1> | number`; upstream collapses the a-site's inner generic \
`t1` to `number` (so `add<number,number>` reduces away) while this port seals `t1` to `unknown`. \
The leaf functions on the path are faithful 1:1 ports: numericBinopTypeFunction \
(Analysis/src/BuiltinTypeFunctions.cpp:390), generalizeType (Analysis/src/Generalization.cpp:730), \
TypeRemover::process (Generalization.cpp:669, only descends Union/Intersection — not type \
functions, matching C++), the ReduceConstraint force path (ConstraintSolver.cpp:2879) and the \
call-dispatch hasBound/instantiate2 logic (ConstraintSolver.cpp:1801). The divergence is emergent \
from generic-instantiation + generalization + type-function-reduction scheduling, not a single \
mistranslated function; fixing it needs core new-solver generalization changes (out of scope, \
high regression risk against the 2222-test baseline)."]
fn type_infer_generics_infer_generic_function_function_argument_overloaded_pt_1() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local g12: (<T>(T, (T) -> T) -> T) & (<T>(T, T, (T, T) -> T) -> T)

        local a = g12(1, function(x) return x + x end)
        local b = g12(1, 2, function(x, y) return x + y end)
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "number | number",
            to_string_type_id(fixture.require_type_string(&String::from("a")))
        );
        assert_eq!(
            "add<unknown, unknown> | number",
            to_string_type_id(fixture.require_type_string(&String::from("b")))
        );
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "number",
            to_string_type_id(fixture.require_type_string(&String::from("a")))
        );
        assert_eq!(
            "number",
            to_string_type_id(fixture.require_type_string(&String::from("b")))
        );
    }
}
