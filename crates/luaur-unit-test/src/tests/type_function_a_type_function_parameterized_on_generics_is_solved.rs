//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1804:type_function_a_type_function_parameterized_on_generics_is_solved`
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
//!   - type_ref -> enum Polarity (Analysis/include/Luau/Polarity.h)
//!   - type_ref -> record TypeFunctionInstanceType (Analysis/include/Luau/Type.h)
//!   - calls -> method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> enum TypeFunctionInstanceState (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_function_a_type_function_parameterized_on_generics_is_solved

#[cfg(test)]
#[test]
fn type_function_a_type_function_parameterized_on_generics_is_solved() {
    use crate::records::tf_fixture::TfFixture;
    use alloc::string::String;
    use core::ptr::NonNull;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::enums::type_function_instance_state::TypeFunctionInstanceState;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::reduce_type_functions_type_function::reduce_type_functions;
    use luaur_analysis::records::generic_type::GenericType;
    use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;
    use luaur_ast::records::location::Location;

    let mut fixture = TfFixture::default();
    let a = fixture
        .arena
        .add_type(GenericType::generic_type_name_polarity(
            &String::from("A"),
            Polarity::Negative,
        ));
    let b = fixture
        .arena
        .add_type(GenericType::generic_type_name_polarity(
            &String::from("B"),
            Polarity::Negative,
        ));

    let add_func = NonNull::from(&fixture.builtin_types.typeFunctions.add_func);
    let add_ty = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
            add_func,
            alloc::vec![a, b],
            alloc::vec![],
        ),
    );

    let _res = reduce_type_functions(
        add_ty,
        Location::default(),
        NonNull::from(&mut *fixture.tfc),
        false,
    );

    let tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(add_ty).as_ref() }
        .expect("expected TypeFunctionInstanceType");
    assert_eq!(TypeFunctionInstanceState::Solved, tfit.state());
}
