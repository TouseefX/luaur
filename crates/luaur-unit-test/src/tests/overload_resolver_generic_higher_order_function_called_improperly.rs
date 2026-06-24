//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/OverloadResolver.test.cpp:280:overload_resolver_generic_higher_order_function_called_improperly`
//! Source: `tests/OverloadResolver.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/OverloadResolver.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/OverloadResolver.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/UnifierSharedState.h
//! - incoming:
//!   - declares <- source_file tests/OverloadResolver.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum Polarity (Analysis/include/Luau/Polarity.h)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record GenericTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record OverloadResolver (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolverFixture::mkResolver (tests/OverloadResolver.test.cpp)
//!   - type_ref -> record OverloadResolution (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolver::resolveOverload (Analysis/src/OverloadResolver.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item overload_resolver_generic_higher_order_function_called_improperly

use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_analysis::enums::polarity::Polarity;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::generic_type::GenericType;
use luaur_analysis::records::generic_type_pack::GenericTypePack;
use luaur_ast::records::location::Location;

#[cfg(test)]
#[test]
fn overload_resolver_generic_higher_order_function_called_improperly() {
    let fixture = OverloadResolverFixture::new();

    let generic_a = unsafe {
        (*fixture.arena).add_type(GenericType::generic_type_name_polarity(
            &"A".to_string(),
            Polarity::Mixed,
        ))
    };
    let generic_bs =
        unsafe { (*fixture.arena).add_type_pack_t(GenericTypePack::new_name("B".to_string())) };
    let generic_cs =
        unsafe { (*fixture.arena).add_type_pack_t(GenericTypePack::new_name("C".to_string())) };

    let function_argument_args = unsafe {
        (*fixture.arena).add_type_pack_vector_type_id_optional_type_pack_id(
            alloc::vec![generic_a],
            Some(generic_bs),
        )
    };
    let function_argument = unsafe {
        (*fixture.arena).add_type(FunctionType::function_type_new(
            function_argument_args,
            generic_cs,
            None,
            false,
        ))
    };

    let apply_args = fixture.pack_initializer_list_type_id(&[function_argument, generic_a]);
    let apply_ty = unsafe {
        (*fixture.arena).add_type(FunctionType::new_with_generics(
            alloc::vec![generic_a],
            alloc::vec![generic_bs, generic_cs],
            apply_args,
            generic_cs,
            None,
            false,
        ))
    };

    let call_args_pack = fixture.pack_initializer_list_type_id(&[
        fixture.number_number_to_number,
        fixture.builtin_types.numberType,
    ]);

    let mut resolver = fixture.mk_resolver();
    let resolution = resolver.resolve_overload(
        apply_ty,
        call_args_pack,
        Location::default(),
        fixture.empty_set,
        false,
    );

    assert_eq!(1, resolution.ok.len());
}
