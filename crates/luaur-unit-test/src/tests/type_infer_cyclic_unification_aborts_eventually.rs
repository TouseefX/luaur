//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2028:type_infer_cyclic_unification_aborts_eventually`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record CodeTooComplex (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_cyclic_unification_aborts_eventually

#[cfg(test)]
#[test]
fn type_infer_cyclic_unification_aborts_eventually() {
    use crate::functions::has_error::has_error;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::records::code_too_complex::CodeTooComplex;
    use luaur_common::{FFlag, FInt};

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let _instantiate_in_subtyping = ScopedFastFlag::new(&FFlag::LuauInstantiateInSubtyping, true);
    let _type_pack_loop_limit = ScopedFastInt::new(&FInt::LuauTypeInferTypePackLoopLimit, 100);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(r#"pcall(table.unpack({pcall}))"#),
        None,
    );

    assert!(has_error::<CodeTooComplex>(&result), "{:?}", result.errors);
}
