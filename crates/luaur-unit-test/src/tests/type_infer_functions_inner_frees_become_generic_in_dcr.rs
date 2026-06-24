//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2216:type_infer_functions_inner_frees_become_generic_in_dcr`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_functions_inner_frees_become_generic_in_dcr

#[cfg(test)]
#[test]
fn type_infer_functions_inner_frees_become_generic_in_dcr() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::generic_type::GenericType;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x)
            local z = x
            return x
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let ty = fixture
        .find_type_at_position_position(Position {
            line: 3,
            column: 19,
        })
        .expect("expected type at position");
    assert!(
        unsafe { get_type_id::<GenericType>(follow_type_id(ty)).as_ref() }.is_some(),
        "expected GenericType"
    );
}
