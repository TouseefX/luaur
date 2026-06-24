//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:368:nonstrict_mode_error_in_union_suppresses`
//! Source: `tests/NonstrictMode.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NonstrictMode.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/NonstrictMode.test.cpp
//! - outgoing:
//!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item nonstrict_mode_error_in_union_suppresses

#[cfg(test)]
#[test]
fn nonstrict_mode_error_in_union_suppresses() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_ast::enums::mode::Mode;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_mode_string_optional_frontend_options(
        Mode::Nonstrict,
        &String::from(
            r#"
        local sublist: any
        if sublist then
            local subitem = sublist.item
            local _ = string.upper(subitem)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
