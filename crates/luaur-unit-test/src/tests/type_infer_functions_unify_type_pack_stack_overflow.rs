//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3968:type_infer_functions_unify_type_pack_stack_overflow`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypePackMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_unify_type_pack_stack_overflow

#[cfg(test)]
#[test]
fn type_infer_functions_unify_type_pack_stack_overflow() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use luaur_analysis::records::type_pack_mismatch::TypePackMismatch;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function a(): ...string
            return "hello", "world"
        end

        local function g<T...>()
            local function f(... : T...)
            end
            f("what", "is", "going", a())
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    let err = unsafe { get_type_error::<TypePackMismatch>(&results.errors[0]).as_ref() }
        .expect("expected TypePackMismatch");
    assert_eq!("T...", to_string_type_pack_id(err.wanted_tp()));
    assert_eq!(
        "string, string, string, ...string",
        to_string_type_pack_id(err.given_tp())
    );
}
