//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Method:Luau.UnitTest:tests/Fixture.cpp:799:is_subtype_fixture_is_subtype`
//! Source: `tests/Fixture.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Fixture.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Constraint.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Ast/include/Luau/PrettyPrinter.h
//!   - includes -> source_file Analysis/include/Luau/Subtyping.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeAttach.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//! - incoming:
//!   - declares <- source_file tests/Fixture.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - calls -> method Module::hasModuleScope (Analysis/src/Module.cpp)
//!   - type_ref -> record UnifierSharedState (Analysis/include/Luau/UnifierSharedState.h)
//!   - calls -> method Module::getModuleScope (Analysis/src/Module.cpp)
//!   - type_ref -> record Normalizer (Analysis/include/Luau/Normalize.h)
//!   - type_ref -> record Unifier (Analysis/include/Luau/Unifier.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record TypeCheckLimits (Analysis/include/Luau/TypeCheckLimits.h)
//!   - type_ref -> record TypeFunctionRuntime (Analysis/include/Luau/TypeFunctionRuntime.h)
//!   - type_ref -> record Subtyping (Analysis/include/Luau/Subtyping.h)
//!   - type_ref -> record IsSubtypeFixture (tests/Fixture.h)
//!   - translates_to -> rust_item IsSubtypeFixture::isSubtype

use crate::records::is_subtype_fixture::IsSubtypeFixture;
use alloc::sync::Arc;
use luaur_analysis::enums::solver_mode::SolverMode;
use luaur_analysis::functions::is_subtype_normalize_alt_b::is_subtype;
use luaur_analysis::records::normalizer::Normalizer;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_analysis::records::type_function_runtime::TypeFunctionRuntime;
use luaur_analysis::records::unifier_shared_state::UnifierSharedState;
use luaur_analysis::type_aliases::type_id::TypeId;

impl IsSubtypeFixture {
    pub fn is_subtype(&mut self, a: TypeId, b: TypeId) -> bool {
        let module = self.base.get_main_module(false);
        assert!(!module.is_null(), "isSubtype: expected main module");

        let module = unsafe { &*module };
        assert!(
            module.has_module_scope(),
            "isSubtype: module scope data is not available"
        );

        let scope = module.get_module_scope();
        let mut shared_state =
            UnifierSharedState::unifier_shared_state(&mut self.base.ice as *mut _);
        let solver_mode = if luaur_common::FFlag::DebugLuauForceOldSolver.get() {
            SolverMode::Old
        } else {
            SolverMode::New
        };
        let mut normalizer = Normalizer::new(
            &mut self.base.arena as *mut _,
            self.base.builtin_types,
            &mut shared_state as *mut _,
            solver_mode,
            false,
        );

        let mut arena = TypeArena::default();
        let limits = TypeCheckLimits::default();
        let mut type_function_runtime =
            TypeFunctionRuntime::new(&self.base.ice, &limits, scope.clone());

        is_subtype(
            a,
            b,
            &mut arena as *mut _,
            self.base.builtin_types,
            Arc::as_ptr(&scope) as *mut _,
            &mut normalizer as *mut _,
            &mut type_function_runtime as *mut _,
            &mut self.base.ice as *mut _,
        )
    }
}
