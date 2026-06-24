//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2730:type_infer_fuzz_avoid_singleton_union`
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
//!   - translates_to -> rust_item type_infer_fuzz_avoid_singleton_union

#[cfg(test)]
#[test]
// Separate subsystem (NOT the for-in iterator cluster): on this fuzz input the
// type checker builds a *structurally self-referential* UnionType (`U = {T, X}`
// where one option follows back to `U` itself — a membership cycle, not a
// `BoundType` chain, so `follow`'s Floyd cycle-detector cannot see it). Any
// recursive type predicate that descends union options — e.g. `is_string`
// (Analysis/src/Type.cpp:199, a faithful 1:1 of C++ `isString`'s
// `std::all_of(begin(utv), end(utv), isString)`) — then recurses forever and
// overflows the stack. C++ avoids this only by never constructing such a union
// for this input; the defect is in the cyclic-union *construction* during the
// fuzzed `if/elseif`/`setmetatable` expression check, a different subsystem from
// for-in iteration. (Previously latent: the old-solver `check(AstStatForIn)` was
// a no-op stub, so iteration never reached `findMetatableEntry`/`isString` on the
// cyclic type; now that the for-in check is faithfully ported, the pre-existing
// hazard becomes reachable.) Re-ignored with precise cause per the task's
// separate-subsystem allowance; fixing it requires de-cycling union construction.
fn type_infer_fuzz_avoid_singleton_union() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        _ = if true then _ else {},if (_) then _ elseif "" then {} elseif _ then {} elseif _ then _ else {}
        for l0,l2 in setmetatable(_,_),l0,_ do
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
