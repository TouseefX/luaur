//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/OverloadResolver.test.cpp:118:overload_resolver_new_basic_overload_selection_1`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - type_ref -> record OverloadResolution (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolver::resolveOverload (Analysis/src/OverloadResolver.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item overload_resolver_new_basic_overload_selection_1

use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_ast::records::location::Location;

#[cfg(test)]
#[test]
fn overload_resolver_new_basic_overload_selection1() {
    let mut fixture = OverloadResolverFixture::new();

    let args = fixture.pack_initializer_list_type_id(&[fixture.builtin_types.stringType]);
    let result = fixture.resolver.resolve_overload(
        fixture.number_to_number_and_string_to_string,
        args,
        Location::default(),
        fixture.empty_set,
        false,
    );

    assert_eq!(1, result.ok.len());
    assert_eq!(fixture.string_to_string, result.ok[0]);

    assert_eq!(1, result.incompatible_overloads.len());
    assert_eq!(fixture.number_to_number, result.incompatible_overloads[0].0);
}
