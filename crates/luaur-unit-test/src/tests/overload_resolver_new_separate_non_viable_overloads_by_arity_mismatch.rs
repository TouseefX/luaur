//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/OverloadResolver.test.cpp:218:overload_resolver_new_separate_non_viable_overloads_by_arity_mismatch`
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
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record OverloadResolution (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolver::resolveOverload (Analysis/src/OverloadResolver.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item overload_resolver_new_separate_non_viable_overloads_by_arity_mismatch

use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_analysis::records::type_pack::TypePack;
use luaur_ast::records::location::Location;

#[cfg(test)]
#[test]
fn overload_resolver_new_separate_non_viable_overloads_by_arity_mismatch() {
    let mut fixture = OverloadResolverFixture::new();
    let _args = TypePack::new(alloc::vec![fixture.builtin_types.stringType], None);

    let ty = fixture.meet_initializer_list_type_id(&[
        fixture.number_to_number,
        fixture.number_to_string,
        fixture.number_number_to_number,
    ]);
    let args = fixture.pack_initializer_list_type_id(&[fixture.builtin_types.stringType]);
    let resolution =
        fixture
            .resolver
            .resolve_overload(ty, args, Location::default(), fixture.empty_set, false);

    assert!(resolution.ok.is_empty());
    assert!(resolution.non_functions.is_empty());
    assert_eq!(1, resolution.arity_mismatches.len());
    assert_eq!(
        fixture.number_number_to_number,
        resolution.arity_mismatches[0]
    );

    assert_eq!(2, resolution.incompatible_overloads.len());
    let number_to_number_found = resolution
        .incompatible_overloads
        .iter()
        .any(|(ty, _)| *ty == fixture.number_to_number);
    let number_to_string_found = resolution
        .incompatible_overloads
        .iter()
        .any(|(ty, _)| *ty == fixture.number_to_string);
    assert!(number_to_number_found);
    assert!(number_to_string_found);
}
