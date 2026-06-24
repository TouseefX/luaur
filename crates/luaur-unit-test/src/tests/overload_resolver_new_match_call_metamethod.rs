//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/OverloadResolver.test.cpp:132:overload_resolver_new_match_call_metamethod`
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
//!   - calls -> method OverloadResolverFixture::fn (tests/OverloadResolver.test.cpp)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - calls -> method OverloadResolverFixture::tableWithCall (tests/OverloadResolver.test.cpp)
//!   - type_ref -> record OverloadResolution (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolver::resolveOverload (Analysis/src/OverloadResolver.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - calls -> function matches (Analysis/include/Luau/ControlFlow.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> record overloaded (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item overload_resolver_new_match_call_metamethod

use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_ast::records::location::Location;

#[cfg(test)]
#[test]
fn overload_resolver_new_match_call_metamethod() {
    let mut fixture = OverloadResolverFixture::new();

    let call_mm = fixture.fn_item(
        &[
            fixture.builtin_types.unknownType,
            fixture.builtin_types.numberType,
        ],
        &[fixture.builtin_types.numberType],
    );
    let tbl = fixture.table_with_call(call_mm);
    let args = fixture.pack_initializer_list_type_id(&[fixture.builtin_types.numberType]);

    let result =
        fixture
            .resolver
            .resolve_overload(tbl, args, Location::default(), fixture.empty_set, false);

    assert_eq!(1, result.ok.len());
    assert_eq!(call_mm, result.ok[0]);
}
