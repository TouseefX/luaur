//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:290:type_infer_type_packs_type_alias_type_packs`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_type_packs

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_type_packs() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Packed<T...> = (T...) -> T...
local a: Packed<>
local b: Packed<number>
local c: Packed<string, number>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let packed = fixture
        .lookup_type(&String::from("Packed"))
        .expect("expected type alias Packed");
    assert_eq!("(T...) -> (T...)", to_string_type_id(packed));
    assert_eq!(
        "() -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "(number) -> number",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "(string, number) -> (string, number)",
        to_string_type_id(fixture.require_type_string(&String::from("c")))
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
-- (U..., T) cannot be parsed right now
type Packed<T, U...> = { f: (a: T, U...) -> (T, U...) }
local a: Packed<number>
local b: Packed<string, number>
local c: Packed<string, number, boolean>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let packed = fixture
        .lookup_type(&String::from("Packed"))
        .expect("expected type alias Packed");
    assert_eq!("Packed<T, U...>", to_string_type_id(packed));
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ f: (T, U...) -> (T, U...) }",
        to_string_type_id_to_string_options(packed, &mut opts)
    );

    let a = fixture.require_type_string(&String::from("a"));
    let a_table = unsafe { get_type_id::<TableType>(a) };
    assert!(!a_table.is_null(), "expected TableType for a");
    assert_eq!("Packed<number>", to_string_type_id(a));
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ f: (number) -> number }",
        to_string_type_id_to_string_options(a, &mut opts)
    );

    let a_table = unsafe { &*a_table };
    assert_eq!(1, a_table.instantiated_type_params.len());
    assert_eq!(1, a_table.instantiated_type_pack_params.len());
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "number",
        to_string_type_id_to_string_options(a_table.instantiated_type_params[0], &mut opts)
    );
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "()",
        to_string_type_pack_id_to_string_options(
            a_table.instantiated_type_pack_params[0],
            &mut opts
        )
    );

    let b = fixture.require_type_string(&String::from("b"));
    let b_table = unsafe { get_type_id::<TableType>(b) };
    assert!(!b_table.is_null(), "expected TableType for b");
    assert_eq!("Packed<string, number>", to_string_type_id(b));
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ f: (string, number) -> (string, number) }",
        to_string_type_id_to_string_options(b, &mut opts)
    );

    let b_table = unsafe { &*b_table };
    assert_eq!(1, b_table.instantiated_type_params.len());
    assert_eq!(1, b_table.instantiated_type_pack_params.len());
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "string",
        to_string_type_id_to_string_options(b_table.instantiated_type_params[0], &mut opts)
    );
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "number",
        to_string_type_pack_id_to_string_options(
            b_table.instantiated_type_pack_params[0],
            &mut opts
        )
    );

    let c = fixture.require_type_string(&String::from("c"));
    let c_table = unsafe { get_type_id::<TableType>(c) };
    assert!(!c_table.is_null(), "expected TableType for c");
    assert_eq!("Packed<string, number, boolean>", to_string_type_id(c));
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ f: (string, number, boolean) -> (string, number, boolean) }",
        to_string_type_id_to_string_options(c, &mut opts)
    );

    let c_table = unsafe { &*c_table };
    assert_eq!(1, c_table.instantiated_type_params.len());
    assert_eq!(1, c_table.instantiated_type_pack_params.len());
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "string",
        to_string_type_id_to_string_options(c_table.instantiated_type_params[0], &mut opts)
    );
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "number, boolean",
        to_string_type_pack_id_to_string_options(
            c_table.instantiated_type_pack_params[0],
            &mut opts
        )
    );
}
