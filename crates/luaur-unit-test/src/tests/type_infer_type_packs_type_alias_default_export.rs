//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:835:type_infer_type_packs_type_alias_default_export`
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
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_export

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_default_export() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("Module/Types"),
        String::from(
            r#"
export type A<T, U = string> = { a: T, b: U }
export type B<T, U = T> = { a: T, b: U }
export type C<T, U = (T, T) -> string> = { a: T, b: U }
export type D<T, U = T, V = U> = { a: T, b: U, c: V }
export type E<T... = (string, number)> = { a: (T...) -> () }
export type F<T, U... = ...T> = { a: T, b: (U...) -> T }
export type G<T..., U... = ()> = { b: (U...) -> T... }
export type H<T... = ()> = { b: (T...) -> T... }
return {}
    "#,
        ),
    );

    let result_types = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/Types"), None);
    assert_eq!(0, result_types.errors.len(), "{:?}", result_types.errors);

    fixture.base.file_resolver.source.insert(
        String::from("Module/Users"),
        String::from(
            r#"
local Types = require(script.Parent.Types)

local a: Types.A<number>
local b: Types.B<number>
local c: Types.C<number>
local d: Types.D<number>
local e: Types.E<>
local eVoid: Types.E<()>
local f: Types.F<number>
local g: Types.G<...number>
local h: Types.H<>
    "#,
        ),
    );

    let result_users = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/Users"), None);
    assert_eq!(0, result_users.errors.len(), "{:?}", result_users.errors);

    for (name, expected) in [
        ("a", "A<number, string>"),
        ("b", "B<number, number>"),
        ("c", "C<number, (number, number) -> string>"),
        ("d", "D<number, number, number>"),
        ("e", "E<string, number>"),
        ("eVoid", "E<>"),
        ("f", "F<number, ...number>"),
        ("g", "G<...number, ()>"),
        ("h", "H<>"),
    ] {
        assert_eq!(
            expected,
            to_string_type_id(
                fixture
                    .base
                    .require_type_module_name_string("Module/Users", &String::from(name))
            ),
            "{name}"
        );
    }
}
