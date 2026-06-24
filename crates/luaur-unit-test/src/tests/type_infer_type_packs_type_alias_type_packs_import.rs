//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:354:type_infer_type_packs_type_alias_type_packs_import`
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
//!   - calls -> method Fixture::lookupImportedType (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_packs_import

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_type_packs_import() {
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

    let a_result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local Import = require(game.A)
local a: Import.Packed<number>
local b: Import.Packed<string, number>
local c: Import.Packed<string, number, boolean>
local d: { a: typeof(c) }
    "#,
        ),
        None,
    );
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let packed = fixture
        .base
        .lookup_imported_type(&String::from("Import"), &String::from("Packed"))
        .expect("expected imported type Import.Packed");
    assert_eq!("Packed<T, U...>", to_string_type_id(packed));

    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ a: T, b: (U...) -> () }",
        to_string_type_id_to_string_options(packed, &mut opts)
    );

    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ a: number, b: () -> () }",
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("a")),
            &mut opts,
        )
    );
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ a: string, b: (number) -> () }",
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("b")),
            &mut opts,
        )
    );
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ a: string, b: (number, boolean) -> () }",
        to_string_type_id_to_string_options(
            fixture.base.require_type_string(&String::from("c")),
            &mut opts,
        )
    );
    assert_eq!(
        "{ a: Packed<string, number, boolean> }",
        to_string_type_id(fixture.base.require_type_string(&String::from("d")))
    );
}
