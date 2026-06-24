//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1852:type_function_reduce_degenerate_refinement`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
//!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
//!   - calls -> function emplaceType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_function_reduce_degenerate_refinement

#[cfg(test)]
#[test]
fn type_function_reduce_degenerate_refinement() {
    use crate::records::tf_fixture::TfFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use core::ptr::NonNull;
    use luaur_analysis::functions::as_mutable_type::as_mutable_type_id;
    use luaur_analysis::functions::reduce_type_functions_type_function::reduce_type_functions;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::blocked_type::BlockedType;
    use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;
    use luaur_analysis::type_aliases::type_variant::TypeVariant;
    use luaur_ast::records::location::Location;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = TfFixture::default();

    let root = fixture.arena.add_type(BlockedType::default());
    let refine_func = NonNull::from(&fixture.builtin_types.typeFunctions.refine_func);
    let unknown_type = fixture.builtin_types.unknownType;
    let refinement = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            refine_func,
            alloc::vec![root, unknown_type],
            alloc::vec![],
        ),
    );

    unsafe {
        (*as_mutable_type_id(root)).ty = TypeVariant::Bound(refinement);
    }
    let _res = reduce_type_functions(
        refinement,
        Location::default(),
        NonNull::from(&mut *fixture.tfc),
        true,
    );

    assert_eq!("unknown", to_string_type_id(refinement));
}
