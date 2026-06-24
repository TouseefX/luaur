//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/OverloadResolver.test.cpp:189:overload_resolver_new_overloads_with_different_arities`
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
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - type_ref -> record OverloadResolution (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolver::resolveOverload (Analysis/src/OverloadResolver.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item overload_resolver_new_overloads_with_different_arities

use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_ast::records::location::Location;

#[cfg(test)]
#[test]
fn overload_resolver_new_overloads_with_different_arities() {
    let mut fixture = OverloadResolverFixture::new();

    let args = fixture.pack_initializer_list_type_id(&[fixture.builtin_types.numberType]);
    let result = fixture.resolver.resolve_overload(
        fixture.number_to_number_and_number_number_to_number,
        args,
        Location::default(),
        fixture.empty_set,
        false,
    );

    assert_eq!(1, result.ok.len());
    assert_eq!(fixture.number_to_number, result.ok[0]);

    assert_eq!(1, result.arity_mismatches.len());
    assert_eq!(fixture.number_number_to_number, result.arity_mismatches[0]);
}
