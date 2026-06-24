//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:684:type_infer_refinements_free_type_is_equal_to_an_lvalue`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record UnifierSharedState (Analysis/include/Luau/UnifierSharedState.h)
//!   - calls -> method RefinementExternTypeFixture::getFrontend (tests/TypeInfer.refinements.test.cpp)
//!   - type_ref -> record Normalizer (Analysis/include/Luau/Normalize.h)
//!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - calls -> method NormalizeFixture::typeFromNormal (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_free_type_is_equal_to_an_lvalue

#[cfg(test)]
#[test]
fn type_infer_refinements_free_type_is_equal_to_an_lvalue() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::normalizer::Normalizer;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::unifier_shared_state::UnifierSharedState;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(a, b: string?)
            if a == b then
                local foo, bar = a, b
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "unknown",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
        );

        let (builtin_types, ice_handler) = {
            let frontend = fixture.get_frontend();
            (
                frontend.builtin_types,
                &mut frontend.ice_handler
                    as *mut luaur_analysis::records::internal_error_reporter::InternalErrorReporter,
            )
        };
        let mut arena = TypeArena::default();
        let mut state = UnifierSharedState::unifier_shared_state(ice_handler);
        let mut normalizer = Normalizer::new(
            &mut arena as *mut TypeArena,
            builtin_types,
            &mut state as *mut UnifierSharedState,
            SolverMode::New,
            false,
        );
        let ty = fixture.require_type_at_position_position(Position::new(3, 36));
        let normalized = normalizer.normalize(ty);
        assert_eq!(
            "string?",
            to_string_type_id(normalizer.type_from_normal(normalized.as_ref()))
        );
    } else {
        assert_eq!(
            "a",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
        );
        assert_eq!(
            "string?",
            to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 36)))
        );
    }
}
