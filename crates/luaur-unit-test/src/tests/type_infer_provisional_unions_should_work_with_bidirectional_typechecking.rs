//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1449:type_infer_provisional_unions_should_work_with_bidirectional_typechecking`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function match (VM/src/lstrlib.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_provisional_unions_should_work_with_bidirectional_typechecking

#[cfg(test)]
#[test]
fn type_infer_provisional_unions_should_work_with_bidirectional_typechecking() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type dog = { name: string }
        local function bark(arg: { [dog]: dog | { left: dog?, right: dog? } })
            -- do something
            return arg
        end

        local molly: dog = { name = "molly" }
        local draco: dog = { name = "draco" }
        local cindy: dog = { name = "cindy" }
        local laika: dog = { name = "laika" }

        -- this should work because they should match with the left-right dog variant with optionals!
        bark{ [molly] = { left = laika }, [draco] = { right = cindy } }
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<TypeMismatch>(&result.errors[0])
        .unwrap_or_else(|| panic!("expected TypeMismatch, got {:?}", result.errors[0]));
    type_error_data_ref::<TypeMismatch>(&result.errors[1])
        .unwrap_or_else(|| panic!("expected TypeMismatch, got {:?}", result.errors[1]));
}
