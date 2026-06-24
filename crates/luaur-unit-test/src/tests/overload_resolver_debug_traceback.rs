//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/OverloadResolver.test.cpp:301:overload_resolver_debug_traceback`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method OverloadResolverFixture::fn (tests/OverloadResolver.test.cpp)
//!   - type_ref -> record OverloadResolver (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolverFixture::mkResolver (tests/OverloadResolver.test.cpp)
//!   - type_ref -> record OverloadResolution (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolver::resolveOverload (Analysis/src/OverloadResolver.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item overload_resolver_debug_traceback

use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

#[cfg(test)]
#[test]
fn overload_resolver_debug_traceback() {
    let make_debug_traceback = |fixture: &OverloadResolverFixture| -> TypeId {
        let overload1 = fixture.fn_item(
            &[
                fixture.builtin_types.optionalStringType,
                fixture.builtin_types.optionalNumberType,
            ],
            &[fixture.builtin_types.stringType],
        );
        let overload2 = fixture.fn_item(
            &[
                fixture.builtin_types.threadType,
                fixture.builtin_types.optionalStringType,
                fixture.builtin_types.optionalNumberType,
            ],
            &[fixture.builtin_types.stringType],
        );
        fixture.meet_initializer_list_type_id(&[overload1, overload2])
    };

    {
        let fixture = OverloadResolverFixture::new();
        let debug_traceback = make_debug_traceback(&fixture);
        let mut resolver = fixture.mk_resolver();
        let resolution = resolver.resolve_overload(
            debug_traceback,
            fixture.builtin_types.emptyTypePack,
            Location::default(),
            fixture.empty_set,
            false,
        );
        assert_eq!(1, resolution.ok.len());
    }

    {
        let fixture = OverloadResolverFixture::new();
        let debug_traceback = make_debug_traceback(&fixture);
        let mut resolver = fixture.mk_resolver();
        let args = fixture.pack_initializer_list_type_id(&[fixture.builtin_types.stringType]);
        let resolution = resolver.resolve_overload(
            debug_traceback,
            args,
            Location::default(),
            fixture.empty_set,
            false,
        );
        assert_eq!(1, resolution.ok.len());
    }

    {
        let fixture = OverloadResolverFixture::new();
        let debug_traceback = make_debug_traceback(&fixture);
        let mut resolver = fixture.mk_resolver();
        let args = fixture.pack_initializer_list_type_id(&[
            fixture.builtin_types.stringType,
            fixture.builtin_types.numberType,
        ]);
        let resolution = resolver.resolve_overload(
            debug_traceback,
            args,
            Location::default(),
            fixture.empty_set,
            false,
        );
        assert_eq!(1, resolution.ok.len());
    }

    {
        let fixture = OverloadResolverFixture::new();
        let debug_traceback = make_debug_traceback(&fixture);
        let mut resolver = fixture.mk_resolver();
        let args = fixture.pack_initializer_list_type_id(&[fixture.builtin_types.threadType]);
        let resolution = resolver.resolve_overload(
            debug_traceback,
            args,
            Location::default(),
            fixture.empty_set,
            false,
        );
        assert_eq!(1, resolution.ok.len());
    }

    {
        let fixture = OverloadResolverFixture::new();
        let debug_traceback = make_debug_traceback(&fixture);
        let mut resolver = fixture.mk_resolver();
        let args = fixture.pack_initializer_list_type_id(&[
            fixture.builtin_types.threadType,
            fixture.builtin_types.stringType,
        ]);
        let resolution = resolver.resolve_overload(
            debug_traceback,
            args,
            Location::default(),
            fixture.empty_set,
            false,
        );
        assert_eq!(1, resolution.ok.len());
    }

    {
        let fixture = OverloadResolverFixture::new();
        let debug_traceback = make_debug_traceback(&fixture);
        let mut resolver = fixture.mk_resolver();
        let args = fixture.pack_initializer_list_type_id(&[
            fixture.builtin_types.threadType,
            fixture.builtin_types.stringType,
            fixture.builtin_types.numberType,
        ]);
        let resolution = resolver.resolve_overload(
            debug_traceback,
            args,
            Location::default(),
            fixture.empty_set,
            false,
        );
        assert_eq!(1, resolution.ok.len());
    }
}
