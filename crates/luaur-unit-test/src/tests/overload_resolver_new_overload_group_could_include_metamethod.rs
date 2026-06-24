//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/OverloadResolver.test.cpp:173:overload_resolver_new_overload_group_could_include_metamethod`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - calls -> method OverloadResolverFixture::tableWithCall (tests/OverloadResolver.test.cpp)
//!   - type_ref -> record OverloadResolution (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolver::resolveOverload (Analysis/src/OverloadResolver.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item overload_resolver_new_overload_group_could_include_metamethod

use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_ast::records::location::Location;

#[cfg(test)]
#[test]
fn overload_resolver_new_overload_group_could_include_metamethod() {
    let mut fixture = OverloadResolverFixture::new();

    let overload1 = fixture.fn_item(
        &[
            fixture.builtin_types.unknownType,
            fixture.builtin_types.numberType,
        ],
        &[fixture.builtin_types.numberType],
    );
    let overload2 = fixture.fn_item(
        &[
            fixture.builtin_types.unknownType,
            fixture.builtin_types.stringType,
        ],
        &[fixture.builtin_types.stringType],
    );
    let tbl = fixture.table_with_call(fixture.meet_type_id_type_id(overload1, overload2));

    let boolean_to_boolean = fixture.fn_item(
        &[fixture.builtin_types.booleanType],
        &[fixture.builtin_types.booleanType],
    );
    let monstrosity = fixture.meet_type_id_type_id(tbl, boolean_to_boolean);
    let args = fixture.pack_initializer_list_type_id(&[fixture.builtin_types.numberType]);

    let result = fixture.resolver.resolve_overload(
        monstrosity,
        args,
        Location::default(),
        fixture.empty_set,
        false,
    );

    assert_eq!(1, result.ok.len());
    assert_eq!(overload1, result.ok[0]);
}
