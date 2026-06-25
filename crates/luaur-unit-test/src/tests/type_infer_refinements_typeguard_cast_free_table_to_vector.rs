//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1445:type_infer_refinements_typeguard_cast_free_table_to_vector`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method RefinementExternTypeFixture::getFrontend (tests/TypeInfer.refinements.test.cpp)
//!   - calls -> method Frontend::setLuauSolverMode (Analysis/src/Frontend.cpp)
//!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias vec (Common/include/Luau/InsertionOrderedMap.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_typeguard_cast_free_table_to_vector

#[cfg(test)]
#[test]
fn type_infer_refinements_typeguard_cast_free_table_to_vector() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::refinement_extern_type_fixture::RefinementExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    // CLI-115286 - Refining via type(x) == 'vector' does not work in the new solver
    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = RefinementExternTypeFixture {
        base: BuiltinsFixture::default(),
    };
    fixture
        .get_frontend()
        .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
            SolverMode::New
        } else {
            SolverMode::Old
        });
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(vec)
            local X, Y, Z = vec.X, vec.Y, vec.Z

            if type(vec) == "vector" then
                local foo = vec
            elseif typeof(vec) == "Instance" then
                local foo = vec
            else
                local foo = vec
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    // type(vec) == "vector"
    assert_eq!(
        "Vector3",
        to_string_type_id(
            fixture
                .base
                .base
                .require_type_at_position_position(Position::new(5, 28))
        )
    );

    // typeof(vec) == "Instance"
    assert_eq!(
        "never",
        to_string_type_id(
            fixture
                .base
                .base
                .require_type_at_position_position(Position::new(7, 28))
        )
    );

    // type(vec) ~= "vector" and typeof(vec) ~= "Instance"
    assert_eq!(
        "{+ X: a, Y: b, Z: c +}",
        to_string_type_id(
            fixture
                .base
                .base
                .require_type_at_position_position(Position::new(9, 28))
        )
    );
}
