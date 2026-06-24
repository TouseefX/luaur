//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1792:type_function_or_a_b`
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
//!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
//!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
//!   - type_ref -> record FunctionGraphReductionResult (Analysis/include/Luau/TypeFunction.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_function_or_a_b

#[cfg(test)]
#[test]
fn type_function_or_a_b() {
    use crate::records::tf_fixture::TfFixture;
    use core::ptr::NonNull;
    use luaur_analysis::functions::reduce_type_functions_type_function::reduce_type_functions;
    use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;
    use luaur_ast::records::location::Location;

    let mut fixture = TfFixture::default();
    let scope = fixture.tfc.scope.as_ptr();
    let a_type = fixture
        .arena
        .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, scope);
    let b_type = fixture
        .arena
        .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, scope);

    let or_func = NonNull::from(&fixture.builtin_types.typeFunctions.or_func);
    let or_type = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            or_func,
            alloc::vec![a_type, b_type],
            alloc::vec![],
        ),
    );

    let res = reduce_type_functions(
        or_type,
        Location::default(),
        NonNull::from(&mut *fixture.tfc),
        false,
    );

    assert_eq!(1, res.reduced_types.size());
}
