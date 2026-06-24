//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1777:type_function_refine_g_false`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> enum Polarity (Analysis/include/Luau/Polarity.h)
//!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
//!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
//!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
//!   - type_ref -> record FunctionGraphReductionResult (Analysis/include/Luau/TypeFunction.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_function_refine_g_false

#[cfg(test)]
#[test]
fn type_function_refine_g_false() {
    use crate::records::tf_fixture::TfFixture;
    use core::ptr::NonNull;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::functions::reduce_type_functions_type_function::reduce_type_functions;
    use luaur_analysis::records::generic_type::GenericType;
    use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;
    use luaur_ast::records::location::Location;

    let mut fixture = TfFixture::default();
    let g = fixture
        .arena
        .add_type(GenericType::generic_type_scope_polarity(
            fixture.tfc.scope.as_ptr(),
            Polarity::Negative,
        ));

    let refine_func = NonNull::from(&fixture.builtin_types.typeFunctions.refine_func);
    let truthy_type = fixture.builtin_types.truthyType;
    let refine_ty = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            refine_func,
            alloc::vec![g, truthy_type],
            alloc::vec![],
        ),
    );

    let res = reduce_type_functions(
        refine_ty,
        Location::default(),
        NonNull::from(&mut *fixture.tfc),
        false,
    );

    assert_eq!(1, res.reduced_types.size());
    assert_eq!(0, res.errors.len(), "{:?}", res.errors);
    assert_eq!(0, res.irreducible_types.size());
    assert_eq!(0, res.blocked_types.size());
}
