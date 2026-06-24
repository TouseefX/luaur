//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:385:type_infer_type_packs_type_pack_type_parameters`
//! Source: `tests/TypeInfer.typePacks.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_type_packs_type_pack_type_parameters

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_pack_type_parameters() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
export type Packed<T, U...> = { a: T, b: (U...) -> () }
return {}
    "#,
        ),
    );

    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local Import = require(game.A)
type Alias<S, T, R...> = Import.Packed<S, (T, R...)>
local a: Alias<string, number, boolean>

type B<X...> = Import.Packed<string, X...>
type C<X...> = Import.Packed<string, (number, X...)>
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let alias = fixture
        .base
        .lookup_type(&String::from("Alias"))
        .expect("expected type alias Alias");
    assert_eq!("Alias<S, T, R...>", to_string_type_id(alias));
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ a: S, b: (T, R...) -> () }",
        to_string_type_id_to_string_options(alias, &mut opts)
    );

    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ a: string, b: (number, boolean) -> () }",
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("a")),
            &mut opts,
        )
    );

    let b = fixture
        .base
        .lookup_type(&String::from("B"))
        .expect("expected type alias B");
    assert_eq!("B<X...>", to_string_type_id(b));
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ a: string, b: (X...) -> () }",
        to_string_type_id_to_string_options(b, &mut opts)
    );

    let c = fixture
        .base
        .lookup_type(&String::from("C"))
        .expect("expected type alias C");
    assert_eq!("C<X...>", to_string_type_id(c));
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ a: string, b: (number, X...) -> () }",
        to_string_type_id_to_string_options(c, &mut opts)
    );
}
