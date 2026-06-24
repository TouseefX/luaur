//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:923:type_infer_extern_types_ice_while_checking_script_due_to_scopes_not_being_solver_agnostic`
//! Source: `tests/TypeInfer.externTypes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
//! - outgoing:
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method Frontend::setLuauSolverMode (Analysis/src/Frontend.cpp)
//!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
//!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_extern_types_ice_while_checking_script_due_to_scopes_not_being_solver_agnostic

#[cfg(test)]
#[test]
fn type_infer_extern_types_ice_while_checking_script_due_to_scopes_not_being_solver_agnostic() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_common::FFlag;

    let _luau_solver_off = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend().set_luau_solver_mode(SolverMode::New);

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local function ExitSeat(player, character, seat, weld)
    --Find vehicle model
    local model
    local newParent = seat
    repeat
        model = newParent
        newParent = model.Parent
    until newParent.ClassName ~= "Model"
    local part, _ = Raycast(seat.Position, dir, dist, {character, model})
end
"#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
