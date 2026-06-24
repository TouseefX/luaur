//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:2061:type_function_reduce_cyclic_add`
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
//!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
//!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
//!   - calls -> function emplaceType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record FunctionGraphReductionResult (Analysis/include/Luau/TypeFunction.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_function_reduce_cyclic_add

#[cfg(test)]
#[test]
fn type_function_reduce_cyclic_add() {
    use crate::records::tf_fixture::TfFixture;
    use core::ptr::NonNull;
    use luaur_analysis::functions::as_mutable_type::as_mutable_type_id;
    use luaur_analysis::functions::reduce_type_functions_type_function::reduce_type_functions;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::blocked_type::BlockedType;
    use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_analysis::type_aliases::type_variant::TypeVariant;
    use luaur_ast::records::location::Location;

    let mut fixture = TfFixture::default();
    let root = fixture.arena.add_type(BlockedType::default());
    let number_type = fixture.builtin_types.numberType;
    let lhs = fixture.arena.add_type(UnionType {
        options: alloc::vec![number_type, root],
    });
    let rhs = fixture.arena.add_type(UnionType {
        options: alloc::vec![number_type, root],
    });
    let add_func = NonNull::from(&fixture.builtin_types.typeFunctions.add_func);
    let add_tfit = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            add_func,
            alloc::vec![lhs, rhs],
            alloc::vec![],
        ),
    );
    unsafe {
        (*as_mutable_type_id(root)).ty = TypeVariant::Bound(add_tfit);
    }

    let res = reduce_type_functions(
        root,
        Location::default(),
        NonNull::from(&mut *fixture.tfc),
        false,
    );

    assert_eq!("number", to_string_type_id(root));
    assert_eq!(3, res.reduced_types.size());
    assert_eq!(0, res.errors.len(), "{:?}", res.errors);
    assert_eq!(0, res.irreducible_types.size());
    assert_eq!(0, res.blocked_types.size());
}
