//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3942:type_infer_functions_oss_2061_modify_visited_generic_ice`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_oss_2061_modify_visited_generic_ice

#[cfg(test)]
#[test]
fn type_infer_functions_oss_2061_modify_visited_generic_ice() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::count_mismatch::CountMismatch;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type actions<T=unknown, A...=...unknown> = { [string]: (state: T, A...) -> (T) }
type disconnect = () -> ()

type producer<state, actions = actions> = {
	get:
		& (() -> state)
		& (<T>(selector: (state) -> T) -> T),
} & actions

type interface = {
	create: <state>(default: state) -> <actions>(actions: actions) -> producer<state, actions>,
}

local a: interface
a.create()
    "#,
        ),
        None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    assert!(unsafe { get_type_error::<CountMismatch>(&results.errors[0]).as_ref() }.is_some());
}
