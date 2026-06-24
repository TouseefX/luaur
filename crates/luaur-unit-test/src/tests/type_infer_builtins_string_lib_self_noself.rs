//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:945:type_infer_builtins_string_lib_self_noself`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function gmatch (VM/src/lstrlib.cpp)
//!   - calls -> function match (VM/src/lstrlib.cpp)
//!   - calls -> function rep (tests/Fixture.cpp)
//!   - calls -> function split (Common/src/StringUtils.cpp)
//!   - translates_to -> rust_item type_infer_builtins_string_lib_self_noself

#[cfg(test)]
#[test]
fn type_infer_builtins_string_lib_self_noself() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        local a1 = string.byte("abcdef", 2)
        local a2 = string.find("abcdef", "def")
        local a3 = string.gmatch("ab ab", "%a+")
        local a4 = string.gsub("abab", "ab", "cd")
        local a5 = string.len("abc")
        local a6 = string.match("12 ab", "%d+ %a+")
        local a7 = string.rep("a", 10)
        local a8 = string.sub("abcd", 1, 2)
        local a9 = string.split("a,b,c", ",")
        local a0 = string.packsize("ff")
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
