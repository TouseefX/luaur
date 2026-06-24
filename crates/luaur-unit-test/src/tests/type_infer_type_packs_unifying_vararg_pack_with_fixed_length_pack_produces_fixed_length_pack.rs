//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:955:type_infer_type_packs_unifying_vararg_pack_with_fixed_length_pack_produces_fixed_length_pack`
//! Source: `tests/TypeInfer.typePacks.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - calls -> method Module::hasModuleScope (Analysis/src/Module.cpp)
//!   - calls -> method Module::getModuleScope (Analysis/src/Module.cpp)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_infer_type_packs_unifying_vararg_pack_with_fixed_length_pack_produces_fixed_length_pack

#[cfg(test)]
#[test]
fn type_infer_type_packs_unifying_vararg_pack_with_fixed_length_pack_produces_fixed_length_pack() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::begin_type_pack::begin;
    use luaur_analysis::functions::end_type_pack::end;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function a(x) return 1 end
        a(...)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let main_module = fixture.get_main_module(false);
    let main_module = unsafe { &*main_module };
    assert!(main_module.has_module_scope());

    let module_scope = main_module.get_module_scope();
    let vararg_pack = module_scope
        .vararg_pack
        .expect("expected module scope vararg pack");

    let mut iter = begin(vararg_pack);
    let end_iter = end(vararg_pack);

    assert!(iter.operator_ne(&end_iter));
    iter.operator_inc();
    assert!(iter.operator_eq(&end_iter));
    assert_eq!(None, iter.tail());
}
