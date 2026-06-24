//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/OverloadResolver.test.cpp:247:overload_resolver_new_select`
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
//!   - calls -> method OverloadResolverFixture::join (tests/OverloadResolver.test.cpp)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record GenericTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record OverloadResolver (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolverFixture::mkResolver (tests/OverloadResolver.test.cpp)
//!   - type_ref -> record OverloadResolution (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolver::resolveOverload (Analysis/src/OverloadResolver.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item overload_resolver_new_select

use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::generic_type_pack::GenericTypePack;
use luaur_ast::records::location::Location;

#[cfg(test)]
#[test]
fn overload_resolver_new_select() {
    let fixture = OverloadResolverFixture::new();

    let number_or_string = fixture.join(
        fixture.builtin_types.numberType,
        fixture.builtin_types.stringType,
    );
    let generic_as =
        unsafe { (*fixture.arena).add_type_pack_t(GenericTypePack::new_name("A".to_string())) };

    let select_args = unsafe {
        (*fixture.arena).add_type_pack_vector_type_id_optional_type_pack_id(
            alloc::vec![number_or_string],
            Some(generic_as),
        )
    };
    let select_ty = unsafe {
        (*fixture.arena).add_type(FunctionType::new_with_generics(
            alloc::vec::Vec::new(),
            alloc::vec![generic_as],
            select_args,
            fixture.builtin_types.anyTypePack,
            None,
            false,
        ))
    };

    let mut resolver = fixture.mk_resolver();
    let call_args = unsafe {
        (*fixture.arena).add_type_pack_vector_type_id_optional_type_pack_id(
            alloc::vec![number_or_string],
            Some(fixture.builtin_types.anyTypePack),
        )
    };
    let resolution = resolver.resolve_overload(
        select_ty,
        call_args,
        Location::default(),
        fixture.empty_set,
        false,
    );

    assert_eq!(1, resolution.ok.len());
}
