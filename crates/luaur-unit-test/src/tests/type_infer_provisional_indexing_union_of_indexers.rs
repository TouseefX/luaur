//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1435:type_infer_provisional_indexing_union_of_indexers`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_provisional_indexing_union_of_indexers

#[cfg(test)]
#[test]
fn type_infer_provisional_indexing_union_of_indexers() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function foo(
            t: { [string]: number } | { [number]: number }
        )
            return t[true]
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
